use std::path::PathBuf;
use jihaz_deferred::message::DeferredTaskMessage;
use masonry::Action;
use tokio::task::JoinHandle;
use xilem::{
    view::{label, Label}, Pod, ViewCtx
};
use xilem_core::{AsyncCtx, MessageProxy, MessageResult, Mut, View, ViewId, ViewMarker, ViewPathTracker};
use crate::{
    app::AppState, 
    deferred::{generate_packages, GeneratePackagesTaskPayload}, 
    app_message::AppMessageResult, 
    progress::ProgressMessage, 
    widget::tablet::TabletWi
};
use super::{button_action::{button, ButtonOfAction}, textbox::{textbox, Textbox}};

pub struct TabletVi {
    pub executable_path_label: Label,
    pub executable_path_textbox: Textbox<AppState, AppMessageResult>,
    pub original_icon_path_label: Label,
    pub original_icon_path_textbox: Textbox<AppState, AppMessageResult>,
    pub target_packages_directory_path_label: Label,
    pub target_packages_directory_path_textbox: Textbox<AppState, AppMessageResult>,
    pub app_name_label: Label,
    pub app_name_textbox: Textbox<AppState, AppMessageResult>,
    // pub generate_app_packages_button: SpecialButton,
    pub generate_app_packages_button: ButtonOfAction<AppState, AppMessageResult>,
    pub progress_message_label: Label,
}

/// The ViewState is retained, unlike View where a new instance is created
/// whenever app_logic is ran.
pub struct TabletViState {
    async_task_payload: Option<GeneratePackagesTaskPayload>,
    async_handle: Option<JoinHandle<()>>,
}

impl TabletViState {
    fn should_run_task(&self) -> bool {
        let lazy = || self.no_running_task();
        self.async_task_payload.is_some() && lazy()
    }

    fn no_running_task(&self) -> bool {
        match self.async_handle.as_ref() {
            Some(h) => h.is_finished(),
            None => true,
        }
    }

    fn discard_finished_handle(&mut self) {
        if self.async_handle.as_ref().map(|h| h.is_finished()).unwrap_or(false) {
            self.async_handle = None;
        }
    }
}

// When the state changes, xilem automatically calls rebuild
impl TabletVi {
    pub fn new(
        executable_path: String,
        original_icon_path: String,
        target_packages_directory_path: String,
        app_name: String,
        progress_message: &ProgressMessage,
    ) -> TabletVi {
        TabletVi {
            executable_path_label: label("Path to app executable:"),
            executable_path_textbox: textbox(executable_path, |app_state: &mut AppState, s| {
                app_state.executable_path = PathBuf::from(s).into();
                AppMessageResult::Nop
            }),
            original_icon_path_label: label("Path to original icon file:"),
            original_icon_path_textbox: textbox(original_icon_path, |app_state: &mut AppState, s| {
                app_state.original_icon_path = PathBuf::from(s).into();
                AppMessageResult::Nop
            }),
            target_packages_directory_path_label: label("Path to the directory to place the generated app packages:"),
            target_packages_directory_path_textbox: textbox(target_packages_directory_path, |app_state: &mut AppState, s| {
                app_state.target_packages_directory_path = PathBuf::from(s).into();
                AppMessageResult::Nop
            }),
            app_name_label: label("Name of the app in small letters"),
            app_name_textbox: textbox(app_name, |app_state: &mut AppState, s| {
                app_state.app_name_lowercase = s;
                AppMessageResult::Nop
            }),
            generate_app_packages_button: 
            button("Generate app packages", |_app_state: &mut AppState| {
                AppMessageResult::GeneratePackages
            }),
            progress_message_label: label(progress_message.to_string()),
        }
    }
}
impl ViewMarker for TabletVi {}
impl View<AppState, AppMessageResult, ViewCtx> for TabletVi {
    type Element = Pod<TabletWi>;
    type ViewState = TabletViState;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        let (exec_path_lbl, _) = ctx.with_id(ViewId::new(1), |ctx| View::<AppState, (), ViewCtx>::build(&self.executable_path_label, ctx));
        let (exec_path_tb, _) = ctx.with_id(ViewId::new(2), |ctx| View::<AppState, AppMessageResult, ViewCtx>::build(&self.executable_path_textbox, ctx));
        let (og_ic_path_lbl, _) = ctx.with_id(ViewId::new(3), |ctx| View::<AppState, (), ViewCtx>::build(&self.original_icon_path_label, ctx));
        let (og_ic_path_tb, _) = ctx.with_id(ViewId::new(4), |ctx| View::<AppState, AppMessageResult, ViewCtx>::build(&self.original_icon_path_textbox, ctx));
        let (tg_pck_dir_path_lbl, _) = ctx.with_id(ViewId::new(5), |ctx| View::<AppState, (), ViewCtx>::build(&self.target_packages_directory_path_label, ctx));
        let (tg_pck_dir_path_tb, _) = ctx.with_id(ViewId::new(6), |ctx| View::<AppState, AppMessageResult, ViewCtx>::build(&self.target_packages_directory_path_textbox, ctx));
        let (app_name_lbl, _) = ctx.with_id(ViewId::new(7), |ctx| View::<AppState, (), ViewCtx>::build(&self.app_name_label, ctx));
        let (app_name_tb, _) = ctx.with_id(ViewId::new(8), |ctx| View::<AppState, AppMessageResult, ViewCtx>::build(&self.app_name_textbox, ctx));
        let (gen_app_pck_btn, _) = ctx.with_id(ViewId::new(9), |ctx| View::<AppState, AppMessageResult, ViewCtx>::build(&self.generate_app_packages_button, ctx));
        let (progress_msg_lbl, _) = ctx.with_id(ViewId::new(7), |ctx| View::<AppState, (), ViewCtx>::build(&self.progress_message_label, ctx));
        
        let element = ctx.with_action_widget(|ctx: &mut ViewCtx| {
            ctx.new_pod(TabletWi::new(
                exec_path_lbl,
                exec_path_tb,
                og_ic_path_lbl,
                og_ic_path_tb,
                tg_pck_dir_path_lbl,
                tg_pck_dir_path_tb,
                app_name_lbl,
                app_name_tb,
                gen_app_pck_btn,
                progress_msg_lbl,
            ))
        });
        let view_state = TabletViState { async_task_payload: None, async_handle: None };
        (element, view_state)
    }

    fn rebuild<'el>(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'el, Self::Element>,
    ) -> Mut<'el, Self::Element> {
        let child_element = element.ctx.get_mut(&mut element.widget.executable_path_label.inner);
        ctx.with_id(ViewId::new(1), |ctx| View::<AppState, (), ViewCtx>::rebuild(&self.executable_path_label, &prev.executable_path_label, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.executable_path_textbox.inner);
        ctx.with_id(ViewId::new(2), |ctx| View::<AppState, AppMessageResult, ViewCtx>::rebuild(&self.executable_path_textbox, &prev.executable_path_textbox, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.original_icon_path_label.inner);
        ctx.with_id(ViewId::new(3), |ctx| View::<AppState, (), ViewCtx>::rebuild(&self.original_icon_path_label, &prev.original_icon_path_label, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.original_icon_path_textbox.inner);
        ctx.with_id(ViewId::new(4), |ctx| View::<AppState, AppMessageResult, ViewCtx>::rebuild(&self.original_icon_path_textbox, &prev.original_icon_path_textbox, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.target_packages_directory_path_label.inner);
        ctx.with_id(ViewId::new(5), |ctx| View::<AppState, (), ViewCtx>::rebuild(&self.target_packages_directory_path_label, &prev.target_packages_directory_path_label, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.target_packages_directory_path_textbox.inner);
        ctx.with_id(ViewId::new(6), |ctx| View::<AppState, AppMessageResult, ViewCtx>::rebuild(&self.target_packages_directory_path_textbox, &prev.target_packages_directory_path_textbox, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.app_name_label.inner);
        ctx.with_id(ViewId::new(7), |ctx| View::<AppState, (), ViewCtx>::rebuild(&self.app_name_label, &prev.app_name_label, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.app_name_textbox.inner);
        ctx.with_id(ViewId::new(8), |ctx| View::<AppState, AppMessageResult, ViewCtx>::rebuild(&self.app_name_textbox, &prev.app_name_textbox, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.generate_app_packages_button.inner);
        ctx.with_id(ViewId::new(9), |ctx| View::<AppState, AppMessageResult, ViewCtx>::rebuild(&self.generate_app_packages_button, &prev.generate_app_packages_button, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.progress_message_label.inner);
        ctx.with_id(ViewId::new(9), |ctx| View::<AppState, AppMessageResult, ViewCtx>::rebuild(&self.progress_message_label, &prev.progress_message_label, &mut (), ctx, child_element));

        if view_state.should_run_task() {

            let payload = view_state.async_task_payload.take().unwrap();
            let proxy = ctx.proxy();
            let path = ctx.view_path().into();
            let progress_message_proxy = MessageProxy::new(proxy.clone(), path);
            let path = ctx.view_path().into();
            let task_message_proxy = MessageProxy::new(proxy, path);

            // println!("started async task");
            view_state.async_handle = Some(ctx.runtime().spawn(async move {
                let _ = generate_packages(payload, &progress_message_proxy, &task_message_proxy);
            }));
        } else {
            if view_state.async_task_payload.is_some() {
                println!("/ < * > < * > / should not run task now, there's a task that's still running.. ");
            }
        }

        element
    }

    fn teardown(
        &self,
        _view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
    ) {
        let child_element = element.ctx.get_mut(&mut element.widget.executable_path_label.inner);
        // ctx.with_id(ViewId::new(1), |ctx| self.executable_path_label.teardown(&mut (), ctx, child_element));
        ctx.with_id(ViewId::new(1), |ctx| View::<AppState, (), ViewCtx>::teardown(&self.executable_path_label, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.executable_path_textbox.inner);
        ctx.with_id(ViewId::new(2), |ctx| View::<AppState, AppMessageResult, ViewCtx>::teardown(&self.executable_path_textbox, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.original_icon_path_label.inner);
        ctx.with_id(ViewId::new(3), |ctx| View::<AppState, (), ViewCtx>::teardown(&self.original_icon_path_label, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.original_icon_path_textbox.inner);
        ctx.with_id(ViewId::new(4), |ctx| View::<AppState, AppMessageResult, ViewCtx>::teardown(&self.original_icon_path_textbox, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.target_packages_directory_path_label.inner);
        ctx.with_id(ViewId::new(5), |ctx| View::<AppState, (), ViewCtx>::teardown(&self.target_packages_directory_path_label, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.target_packages_directory_path_textbox.inner);
        ctx.with_id(ViewId::new(6), |ctx| View::<AppState, AppMessageResult, ViewCtx>::teardown(&self.target_packages_directory_path_textbox, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.app_name_label.inner);
        ctx.with_id(ViewId::new(7), |ctx| View::<AppState, (), ViewCtx>::teardown(&self.app_name_label, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.app_name_textbox.inner);
        ctx.with_id(ViewId::new(8), |ctx| View::<AppState, AppMessageResult, ViewCtx>::teardown(&self.app_name_textbox, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.generate_app_packages_button.inner);
        ctx.with_id(ViewId::new(9), |ctx| View::<AppState, AppMessageResult, ViewCtx>::teardown(&self.generate_app_packages_button, &mut (), ctx, child_element));
        let child_element = element.ctx.get_mut(&mut element.widget.progress_message_label.inner);
        ctx.with_id(ViewId::new(9), |ctx| View::<AppState, AppMessageResult, ViewCtx>::teardown(&self.progress_message_label, &mut (), ctx, child_element));
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[ViewId],
        mut message: xilem_core::DynMessage,
        app_state: &mut AppState,
    ) -> MessageResult<AppMessageResult> {
        
        if id_path.is_empty() {
            // println!("without id in stream");

            match message.downcast::<ProgressMessage>() {
                Ok(progress_message) => {
                    app_state.progress_message.set(*progress_message);
                    view_state.discard_finished_handle();
                    return MessageResult::RequestRebuild
                }
                Err(other_message) => {
                    message = other_message;
                }
            }
            match message.downcast::<DeferredTaskMessage>() {
                Ok(progress_message) => {
                    app_state.progress_message.set((*progress_message).into());
                    view_state.discard_finished_handle();
                    return MessageResult::RequestRebuild
                }
                Err(other_message) => {
                    message = other_message;
                }
            }
            match message.downcast::<Action>() {
                Ok(action) => {
                    if let Action::Other(other) = action.as_ref() {
                        if let Some(_app_message) = other.downcast_ref::<AppMessageResult>() {
                            // if let AppMessageResult::NextProgressMessage = app_message {
                            //     println!("continued stream");
                            //     if let Some(next_message) = progres_stream.next_message() {
                            //         // set the current progress message, this app state change will trigger a rebuild pass.
                            //         app_state.progress_message.set(next_message);
                            //         if progres_stream.has_messages() {
                            //             // feed back the remaining progres stream to the next message pass once
                            //             // the current message is set via View::rebuild and shown to the user.
                            //             MessageResult::Action(AppMessageResult::ShowProgressMessages(progres_stream.clone()))
                            //         } else {
                            //             MessageResult::RequestRebuild
                            //         }
                            //     } else {
                            //         MessageResult::Nop
                            //     }
                            // } else {
                                tracing::error!("Wrong AppMessageResult type found in TabletVi::message");
                                MessageResult::Stale(action)
                            // }
                        } else {
                            tracing::error!("Wrong Action::Other type in TabletVi::message");
                            MessageResult::Stale(action)
                        }
                    } else {
                        tracing::error!("Wrong Action variant in TabletVi::message");
                        MessageResult::Stale(action)
                    }
                }
                
                Err(message) => {
                    tracing::error!("Wrong message type in TabletVi::message");
                    MessageResult::Stale(message)
                }
            }
        } else {
            // println!("with id in stream");
            
            let (start, rest) = id_path
                .split_first()
                .expect("Id path has elements for vector");

            let message_result = match start.routing_id() {
                1 => self.executable_path_label.message(&mut (), rest, message, app_state),
                2 => self.executable_path_textbox.message(&mut (), rest, message, app_state),
                3 => self.original_icon_path_label.message(&mut (), rest, message, app_state),
                4 => self.original_icon_path_textbox.message(&mut (), rest, message, app_state),
                5 => self.target_packages_directory_path_label.message(&mut (), rest, message, app_state),
                6 => self.target_packages_directory_path_textbox.message(&mut (), rest, message, app_state),
                7 => self.app_name_label.message(&mut (), rest, message, app_state),
                8 => self.app_name_textbox.message(&mut (), rest, message, app_state),
                9 => self.generate_app_packages_button.message(&mut (), rest, message, app_state),
                10 => self.progress_message_label.message(&mut (), rest, message, app_state),
                _ => unreachable!(),
            };
            match message_result {
                MessageResult::Action(app_message) => {
                    match app_message {
                        AppMessageResult::GeneratePackages => {
                            if app_state.executable_path.is_none()
                                || app_state.original_icon_path.is_none()
                                || app_state.target_packages_directory_path.is_none()
                                || app_state.app_name_lowercase.is_empty()
                            {
                                app_state.progress_message.set(ProgressMessage::NotAllFieldsAreFilled);
                                return MessageResult::RequestRebuild;
                            }
                            if !app_state.executable_path.as_ref().unwrap().exists() {
                                app_state.progress_message.set(ProgressMessage::ExecutablePathDoesNotPointToAFile);
                                return MessageResult::RequestRebuild;
                            }
                            if !app_state.original_icon_path.as_ref().unwrap().is_file() {
                                app_state.progress_message.set(ProgressMessage::SourceIconPathDoesNotPointToAFile);
                                return MessageResult::RequestRebuild;
                            }
                            if !app_state.target_packages_directory_path.as_ref().unwrap().is_dir() {
                                app_state.progress_message.set(ProgressMessage::TargetDirectoryPathIsNotADirectory);
                                return MessageResult::RequestRebuild;
                            }
                            // rebuilding will create an async task that will complete
                            // the packages generation on another thread on the rebuild method, and will send progress messages via the proxy
                            // to the main thread.
                            view_state.async_task_payload = Some(GeneratePackagesTaskPayload {
                                executable_path: app_state.executable_path.clone().unwrap(),
                                other_executables: Vec::new(),
                                original_icon_path: app_state.original_icon_path.clone().unwrap(),
                                app_name_lower_case: app_state.app_name_lowercase.to_string(),
                                target_directory_path: app_state.target_packages_directory_path.clone().unwrap(),
                            });
                            MessageResult::RequestRebuild
                        }
                        AppMessageResult::Nop => MessageResult::Nop,
                    }
                }
                MessageResult::RequestRebuild => todo!(),
                MessageResult::Nop => MessageResult::Nop,
                MessageResult::Stale(message) => MessageResult::Stale(message),
            }
        }
    }
}
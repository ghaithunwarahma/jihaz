use std::path::PathBuf;
use jihaz::collection::SmallKeyArray;
use jihaz_deferred::message::DeferredTaskMessage;
use tokio::task::JoinHandle;
use xilem::{
    view::{button, label, textbox, Button, Label, PointerButton, Textbox}, Pod, ViewCtx
};
use xilem_core::{
    AsyncCtx, MessageProxy, MessageResult, Mut, 
    View, ViewId, ViewMarker, ViewPathTracker
};
use crate::{
    app::AppState, app_message::AppMessageResult, 
    deferred::{generate_packages, GeneratePackagesTaskPayload}, 
    progress::ProgressMessage, widget::{self, tablet::TabletChild}
};

pub struct Tablet<F> {
    pub labels: SmallKeyArray<TabletChild, Label, 5>,
    pub textboxes: SmallKeyArray<TabletChild, Textbox<AppState, AppMessageResult>, 4>,
    pub buttons: SmallKeyArray<TabletChild, Button<F>, 1>,
}

/// The ViewState is retained, unlike View where a new instance is created
/// whenever app_logic is ran.
pub struct TabletState {
    async_task_payload: Option<GeneratePackagesTaskPayload>,
    async_handle: Option<JoinHandle<()>>,
}

impl TabletState {
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

pub fn tablet(
    executable_path: String,
    original_icon_path: String,
    target_packages_directory_path: String,
    app_name: String,
    progress_message: &ProgressMessage,
) -> Tablet<impl for<'a> Fn(&'a mut AppState, PointerButton) -> MessageResult<AppMessageResult> + Send + 'static> {
    use TabletChild as TC;
    Tablet {
        labels: SmallKeyArray::new([
            (TC::ExecutablePath, label("Path to app executable:")),
            (TC::OriginalIconPath, label("Path to original icon file:")),
            (TC::TargetPckgsDirPath, label("Path to the directory to place the generated app packages:")),
            (TC::AppName, label("Name of the app in small letters")),
            (TC::ProgressMessage, label(progress_message.to_string())),
        ]),
        textboxes: SmallKeyArray::new([
            (TC::ExecutablePath, textbox(executable_path, |app_state: &mut AppState, s| {
                app_state.executable_path = PathBuf::from(s).into();
                AppMessageResult::Nop
            })),
            (TC::OriginalIconPath, textbox(original_icon_path, |app_state: &mut AppState, s| {
                app_state.original_icon_path = PathBuf::from(s).into();
                AppMessageResult::Nop
            })),
            (TC::TargetPckgsDirPath, textbox(target_packages_directory_path, |app_state: &mut AppState, s| {
                app_state.target_packages_directory_path = PathBuf::from(s).into();
                AppMessageResult::Nop
            })),
            (TC::AppName, textbox(app_name, |app_state: &mut AppState, s| {
                app_state.app_name_lowercase = s;
                AppMessageResult::Nop
            })),
        ]),
        buttons: SmallKeyArray::new([
            (TC::GenerateAppPckgs, button("Generate app packages", |_app_state: &mut AppState| {
                AppMessageResult::GeneratePackages
            }))
        ]),
    }
}

// --- MARK: TYPES TO REDUCE DUPLICATION ---
type TabletVS<F> = <Tablet<F> as View<AppState, AppMessageResult, ViewCtx>>::ViewState;
type TabletEl<F> = <Tablet<F> as View<AppState, AppMessageResult, ViewCtx>>::Element;

impl<F> ViewMarker for Tablet<F> {}

impl<F> View<AppState, AppMessageResult, ViewCtx> for Tablet<F> 
where
    F: Fn(&mut AppState, PointerButton) -> MessageResult<AppMessageResult> + Send + Sync + 'static,
{
    type Element = Pod<widget::Tablet>;
    type ViewState = TabletState;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        use TabletChild as TC;

        let labels = SmallKeyArray::new([
            (TC::ExecutablePath, ctx.with_id(ViewId::new(1), |ctx| <Label as View<AppState, (), ViewCtx>>::build(&self.labels.get(&TC::ExecutablePath), ctx).0.into_widget_pod())),
            (TC::OriginalIconPath, ctx.with_id(ViewId::new(2), |ctx| <Label as View<AppState, (), ViewCtx>>::build(&self.labels.get(&TC::OriginalIconPath), ctx).0.into_widget_pod())),
            (TC::TargetPckgsDirPath, ctx.with_id(ViewId::new(3), |ctx| <Label as View<AppState, (), ViewCtx>>::build(&self.labels.get(&TC::TargetPckgsDirPath), ctx).0.into_widget_pod())),
            (TC::AppName, ctx.with_id(ViewId::new(4), |ctx| <Label as View<AppState, (), ViewCtx>>::build(&self.labels.get(&TC::AppName), ctx).0.into_widget_pod())),
            (TC::ProgressMessage, ctx.with_id(ViewId::new(5), |ctx| <Label as View<AppState, (), ViewCtx>>::build(&self.labels.get(&TC::ProgressMessage), ctx).0.into_widget_pod())),
        ]);
        let textboxes = SmallKeyArray::new([
            (TC::ExecutablePath, ctx.with_id(ViewId::new(6), |ctx| self.textboxes.get(&TC::ExecutablePath).build(ctx).0.into_widget_pod())),
            (TC::OriginalIconPath, ctx.with_id(ViewId::new(7), |ctx| self.textboxes.get(&TC::OriginalIconPath).build(ctx).0.into_widget_pod())),
            (TC::TargetPckgsDirPath, ctx.with_id(ViewId::new(8), |ctx| self.textboxes.get(&TC::TargetPckgsDirPath).build(ctx).0.into_widget_pod())),
            (TC::AppName, ctx.with_id(ViewId::new(9), |ctx| self.textboxes.get(&TC::AppName).build(ctx).0.into_widget_pod())),
        ]);
        let buttons = SmallKeyArray::new([
            (TC::GenerateAppPckgs, ctx.with_id(ViewId::new(10), |ctx| self.buttons.get(&TC::GenerateAppPckgs).build(ctx).0.into_widget_pod()))
        ]);
        let element = ctx.with_action_widget(|ctx: &mut ViewCtx| {
            ctx.new_pod(widget::Tablet::new(
                labels,
                textboxes,
                buttons,
            ))
        });
        let view_state = TabletState { async_task_payload: None, async_handle: None };
        (element, view_state)
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        use TabletChild as TC;

        fn rebuild_label<F>(ref key: TabletChild, id: u64, this: &Tablet<F>, prev: &Tablet<F>, _view_state: &mut TabletVS<F>, ctx: &mut ViewCtx, element: &mut Mut<TabletEl<F>>) 
        where
            F: Fn(&mut AppState, PointerButton) -> MessageResult<AppMessageResult> + Send + Sync + 'static,
        {
            ctx.with_id(ViewId::new(id), |ctx| {
                let child_element = widget::Tablet::label_mut(key, element);
                <Label as View<AppState, (), ViewCtx>>::rebuild(this.labels.get(key), prev.labels.get(key), &mut (), ctx, child_element)
            });
        }
        rebuild_label(TC::ExecutablePath, 1, self, prev, view_state, ctx, &mut element);
        rebuild_label(TC::OriginalIconPath, 2, self, prev, view_state, ctx, &mut element);
        rebuild_label(TC::TargetPckgsDirPath, 3, self, prev, view_state, ctx, &mut element);
        rebuild_label(TC::AppName, 4, self, prev, view_state, ctx, &mut element);
        rebuild_label(TC::ProgressMessage, 5, self, prev, view_state, ctx, &mut element);

        fn rebuild_textbox<F>(ref key: TabletChild, id: u64, this: &Tablet<F>, prev: &Tablet<F>, _view_state: &mut TabletVS<F>, ctx: &mut ViewCtx, element: &mut Mut<TabletEl<F>>) 
        where
            F: Fn(&mut AppState, PointerButton) -> MessageResult<AppMessageResult> + Send + Sync + 'static,
        {
            ctx.with_id(ViewId::new(id), |ctx| {
                let child_element = widget::Tablet::textbox_mut(key, element);
                this.textboxes.get(key).rebuild(prev.textboxes.get(key), &mut (), ctx, child_element)
            });
        }
        rebuild_textbox(TC::ExecutablePath, 6, self, prev, view_state, ctx, &mut element);
        rebuild_textbox(TC::OriginalIconPath, 7, self, prev, view_state, ctx, &mut element);
        rebuild_textbox(TC::TargetPckgsDirPath, 8, self, prev, view_state, ctx, &mut element);
        rebuild_textbox(TC::AppName, 9, self, prev, view_state, ctx, &mut element);

        ctx.with_id(ViewId::new(10), |ctx| {
            let child_element = widget::Tablet::button_mut(&TC::GenerateAppPckgs, &mut element);
            self.buttons.get(&TC::GenerateAppPckgs).rebuild(prev.buttons.get(&TC::GenerateAppPckgs), &mut (), ctx, child_element)
        });
        
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
    }

    fn teardown(
        &self,
        _view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<Self::Element>,
    ) {
        use TabletChild as TC;

        fn teardown_label<F>(ref key: TabletChild, id: u64, this: &Tablet<F>, _view_state: &mut TabletVS<F>, ctx: &mut ViewCtx, element: &mut Mut<TabletEl<F>>) 
        where
            F: Fn(&mut AppState, PointerButton) -> MessageResult<AppMessageResult> + Send + Sync + 'static,
        {
            ctx.with_id(ViewId::new(id), |ctx| {
                let child_element = widget::Tablet::label_mut(key, element);
                <Label as View<AppState, (), ViewCtx>>::teardown(this.labels.get(key), &mut (), ctx, child_element)
            });
        }
        teardown_label(TC::ExecutablePath, 1, self, _view_state, ctx, &mut element);
        teardown_label(TC::OriginalIconPath, 2, self, _view_state, ctx, &mut element);
        teardown_label(TC::TargetPckgsDirPath, 3, self, _view_state, ctx, &mut element);
        teardown_label(TC::AppName, 4, self, _view_state, ctx, &mut element);
        teardown_label(TC::ProgressMessage, 5, self, _view_state, ctx, &mut element);

        fn teardown_textbox<F>(ref key: TabletChild, id: u64, this: &Tablet<F>, _view_state: &mut TabletVS<F>, ctx: &mut ViewCtx, element: &mut Mut<TabletEl<F>>) 
        where
            F: Fn(&mut AppState, PointerButton) -> MessageResult<AppMessageResult> + Send + Sync + 'static,
        {
            ctx.with_id(ViewId::new(id), |ctx| {
                let child_element = widget::Tablet::textbox_mut(key, element);
                this.textboxes.get(key).teardown(&mut (), ctx, child_element)
            });
        }
        teardown_textbox(TC::ExecutablePath, 6, self, _view_state, ctx, &mut element);
        teardown_textbox(TC::OriginalIconPath, 7, self, _view_state, ctx, &mut element);
        teardown_textbox(TC::TargetPckgsDirPath, 8, self, _view_state, ctx, &mut element);
        teardown_textbox(TC::AppName, 9, self, _view_state, ctx, &mut element);

        ctx.with_id(ViewId::new(10), |ctx| {
            let child_element = widget::Tablet::button_mut(&TC::GenerateAppPckgs, &mut element);
            self.buttons.get(&TC::GenerateAppPckgs).teardown(&mut (), ctx, child_element)
        });
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
            MessageResult::Stale(message)
        } else {
            // println!("with id in stream");
            
            let (start, rest) = id_path
                .split_first()
                .expect("Id path has elements for vector");

            use TabletChild as TC;

            let message_result = match start.routing_id() {
                1 => self.labels.get(&TC::ExecutablePath).message(&mut (), rest, message, app_state),
                2 => self.labels.get(&TC::OriginalIconPath).message(&mut (), rest, message, app_state),
                3 => self.labels.get(&TC::TargetPckgsDirPath).message(&mut (), rest, message, app_state),
                4 => self.labels.get(&TC::AppName).message(&mut (), rest, message, app_state),
                5 => self.labels.get(&TC::ProgressMessage).message(&mut (), rest, message, app_state),
                6 => self.textboxes.get(&TC::ExecutablePath).message(&mut (), rest, message, app_state),
                7 => self.textboxes.get(&TC::OriginalIconPath).message(&mut (), rest, message, app_state),
                8 => self.textboxes.get(&TC::TargetPckgsDirPath).message(&mut (), rest, message, app_state),
                9 => self.textboxes.get(&TC::AppName).message(&mut (), rest, message, app_state),
                10 => self.buttons.get(&TC::GenerateAppPckgs).message(&mut (), rest, message, app_state),
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
use std::iter::{Filter, FusedIterator, Map};

use scraper::{
    element_ref::Select as SelectInElement, 
    error::SelectorErrorKind, html::Select as SelectInHtml, 
    CaseSensitivity, ElementRef, Html, Selector
};

pub enum PickElement<'a> {
    HtmlBody {
        html: Html,
        selector: Selector,
    },
    Element {
        element: ElementRef<'a>,
        selector: Selector,
    }
}
// When we create the trait instead of pass it as a type, we apply trait bound directly instead of defining trait as a trait type..
// https://stackoverflow.com/questions/73204977/implement-factory-function-that-returns-a-struct-with-closure
impl<'a: 'b, 'b> PickElement<'a> {
    pub fn of_html(html_body: &str, selector: &str) 
    -> Result<PickElement<'a>, PickElementError> {
        Ok(PickElement::HtmlBody {
            html: Html::parse_document(&html_body),
            selector: Selector::parse(selector).map_err(PickElementError::from)?,
        })
    }

    pub fn of_element(element: ElementRef<'a>, selector: &str) 
    -> Result<PickElement<'a>, PickElementError> {
        Ok(PickElement::Element {
            element,
            selector: Selector::parse(selector).map_err(PickElementError::from)?,
        })
    }

    pub fn pick_one_by_class_and_get_attr<V>(
        &'a self,
        class_name: &str,
        attr_name: &str,
    ) -> Result<String, PickElementError> {
        self.pick_one_by_class(class_name)
            .and_then(|el| {
                el.value().attr(attr_name).map(String::from).ok_or_else(|| {
                    PickElementError::AttrNotFound { }
                })
            })
    }

    pub fn pick_one_by_class_and<And, V>(&'a self, class_name: &str, and: And) 
    -> Result<V, PickElementError> 
    where
        And: FnOnce(ElementRef<'a>) -> V
    {
        self.pick_one_by_class(class_name).map(and)
    }

    pub fn pick_one_by_class(&'a self, class_name: &str) 
    -> Result<ElementRef<'a>, PickElementError> {
        self.pick_one_by(|el| el
            .value()
            .has_class(class_name, CaseSensitivity::AsciiCaseInsensitive)
        )
    }

    pub fn pick_one_by_and<By, And, V>(&'a self, by: By, and: And) 
    -> Result<V, PickElementError> 
    where
        By: FnMut(&ElementRef<'a>) -> bool,
        And: FnOnce(ElementRef<'a>) -> V,
    {
        self.pick_one_by(by).map(and)
    }

    pub fn pick_one_by<By>(&'a self, by: By) -> Result<ElementRef<'a>, PickElementError> 
    where
        By: FnMut(&ElementRef<'a>) -> bool,
    {
        self.select().find(by).ok_or_else(|| PickElementError::ElementNotFound {})
    }

    pub fn pick_by_class_and_get_attr<And, V>(
        &'a self,
        class_name: &'a str,
        attr_name: &'a str,
    ) -> Map<Filter<Select<'a, 'b>, impl FnMut(&ElementRef<'a>) -> bool + '_>, impl FnMut(ElementRef<'a>) -> Result<String, PickElementError> + '_> {
        self.pick_by_class(class_name)
            .map(|el| {
                el.value().attr(attr_name).map(String::from).ok_or_else(|| {
                    PickElementError::AttrNotFound { }
                })
            })
    }

    pub fn pick_by_class_and<And, V>(&'a self, class_name: &'a str, and: And) 
    -> Map<Filter<Select<'a, 'b>, impl FnMut(&ElementRef<'a>) -> bool + '_>, And> 
    where
        And: FnMut(ElementRef<'a>) -> V
    {
        self.pick_by_class(class_name).map(and)
    }

    pub fn pick_by_class(&'a self, class_name: &'a str) 
    -> Filter<Select<'a, 'b>, impl FnMut(&ElementRef<'a>) -> bool + '_> {
        self.select().filter(|el| el
            .value()
            .has_class(class_name, CaseSensitivity::AsciiCaseInsensitive)
        )
    }

    pub fn pick_by_into_vec<By>(&'a self, by: By) -> Vec<ElementRef<'a>> 
    where
        By: FnMut(&ElementRef<'a>) -> bool,
    {
        self.select().filter(by).collect::<Vec<_>>()
    }

    pub fn pick_by_and<By, And, V>(&'a self, by: By, and: And) 
    -> Map<Filter<Select<'a, 'b>, By>, And>
    where
        By: FnMut(&ElementRef<'a>) -> bool,
        And: FnMut(ElementRef<'a>) -> V,
    {
        self.select().filter(by).map(and)
    }

    pub fn pick_by<By>(&'a self, by: By) -> Filter<Select<'a, 'b>, By> 
    where
        By: FnMut(&ElementRef<'a>) -> bool,
    {
        self.select().filter(by)
    }

    pub fn select(&'a self) -> Select<'a, 'b> {
        match self {
            PickElement::HtmlBody { html, selector } => {
                Select::from_html(html.select(selector))
            }
            PickElement::Element { element, selector } => {
                Select::from_element(element.select(selector))
            }
        }
    }
}

/// Iterator over elements matching a selector.
#[derive(Clone, Debug)]
pub enum Select<'a, 'b> {
    Html(SelectInHtml<'a, 'b>),
    Element(SelectInElement<'a, 'b>),
}

impl<'a, 'b> Select<'a, 'b> {
    fn from_html(select: SelectInHtml<'a, 'b>) -> Select<'a, 'b> {
        Select::Html(select)
    }
    fn from_element(select: SelectInElement<'a, 'b>) -> Select<'a, 'b> {
        Select::Element(select)
    }
}

impl<'a, 'b> Iterator for Select<'a, 'b> {
    type Item = ElementRef<'a>;

    fn next(&mut self) -> Option<ElementRef<'a>> {
        match self {
            Select::Html(sel) => sel.next(),
            Select::Element(sel) => sel.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Select::Html(sel) => sel.size_hint(),
            Select::Element(sel) => sel.size_hint(),
        }
    }
}

// impl<'a, 'b> DoubleEndedIterator for Select<'a, 'b> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         match self {
//             Select::Html(sel) => sel.next_back(),
//             Select::Element(sel) => sel.next_back(),
//         }
//     }
// }

impl FusedIterator for Select<'_, '_> {}

// impl<'a: 'b, 'b> From<SelectInHtml<'a, 'b>> for Select<'a, 'b> {
//     fn from(value: SelectInHtml) -> Select<'a, 'b> {
//         Select::Html(value)
//     }
// }

// impl<'a: 'b, 'b> From<SelectInElement<'a, 'b>> for Select<'a, 'b> {
//     fn from(value: SelectInElement) -> Select<'a, 'b> {
//         Select::Element(value)
//     }
// }

#[derive(Clone, Debug)]
pub enum PickElementError {
    ElementNotFound {
        // selector: String,
        // class_name: String,
    },
    AttrNotFound {
        // selector: String,
        // class_name: String,
        // attr_name: String,
    },
    // SelectorError(SelectorErrorKind<'b>)
    // removed the lifetime to enable Sync and Send for Erye StdError blanket implementation
    SelectorError(String)
}

impl<'a, 'b> From<SelectorErrorKind<'b>> for PickElementError {
    fn from(err: SelectorErrorKind<'b>) -> PickElementError {
        PickElementError::SelectorError(std::error::Error::description(&err).to_string())
    }
}

impl std::fmt::Display for PickElementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PickElementError::ElementNotFound {  } => write!(f, "Element not found"),
            PickElementError::AttrNotFound {  } => write!(f, "Attribute not found"),
            PickElementError::SelectorError(err) => {
                write!(f, "Selector Error: ")?;
                std::fmt::Display::fmt(err, f)
            }
        }
    }
}

impl std::error::Error for PickElementError {
    // fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    //     match self {
    //         PickElementError::ElementNotFound {  } => None,
    //         PickElementError::AttrNotFound {  } => None,
    //         PickElementError::SelectorError(err) => Some(err),
    //     }
    // }
    fn description(&self) -> &str {
        match self {
            PickElementError::ElementNotFound {  } => "Element not found",
            PickElementError::AttrNotFound {  } => "Element not found",
            // PickElementError::SelectorError(err) => err.description(),
            PickElementError::SelectorError(err) => err.as_str(),
        }
    }
}

// pub struct ElementFinder<'a, 'b> {
//     /// We keep a copy of html to avoid lifetime issues
//     html: Html,
//     // The option is used for ownership management only.
//     stage: ThisStage<'a, 'b>,
// }

// pub enum ThisStage<'a, 'b> {
//     Html,
//     SelectInHtml(Option<SelectInHtml<'a, 'b>>),
//     Element(ElementRef<'a>),
//     SelectInElement(Option<SelectInElement<'a, 'b>>)
// }

// impl<'a, 'b> ElementFinder<'a, 'b> {
//     pub fn new(html_body: &str) -> Self {
//         ElementFinder {
//             html: Html::parse_document(&html_body),
//             stage: ThisStage::Html,
//         }
//     }

//     // /// Gets the attribute of the matching element in the HTML string.
//     // pub fn get_attr(
//     //     &self,
//     //     selector: &str,
//     //     class_name: &str,
//     //     attr_name: &str,
//     // ) -> Result<String, ElementFinderError> {
//     //     self.with_element(selector, class_name, |_, el| {
//     //         el.value().attr(attr_name).map(String::from).ok_or(
//     //             ElementFinderError::AttrNotFound {
//     //                 selector: selector.to_string(), 
//     //                 class_name: class_name.to_string(), 
//     //                 attr_name: attr_name.to_string() 
//     //             })
//     //     })?
//     // }

//     // /// Provides an iterator over the child elements after finding the element.
//     // pub fn with_child_elements<'a, V>(
//     //     &'a mut self,
//     //     selector: &str,
//     //     class_name: &str,
//     //     child_selector: &str,
//     //     with_child_elements: impl Fn(&'a Select<'a, 'a>) -> V,
//     // ) -> Result<V, ElementFinderError> {
//     //     self.with_element(selector, class_name, |cs, el| {
//     //         let selector = Selector::parse(child_selector).unwrap();
//     //         let select_iterator = el.select(&selector);
//     //         let res = with_child_elements(&select_iterator);
//     //         res
//     //     })
//     // }

//     // /// Provides the element that matches has the given selector and class name.
//     // /// 
//     // /// Returns None if no matching element is found.
//     // pub fn with_element<'a, V>(
//     //     &'a mut self,
//     //     selector: &str,
//     //     class_name: &str,
//     //     with_element: impl Fn(Option<&mut Selector>, ElementRef<'a>) -> V,
//     // ) -> Result<V, ElementFinderError> {
//     //     let child_selector = self.child_selector.as_mut();
//     //     Self::get_element(&self.html, selector, class_name)
//     //         .map(|el| with_element(child_selector, el))
//     // }

//     // /// Gets the attribute of the matching element in the HTML string.
//     // pub fn get_attr(
//     //     &self,
//     //     attr_name: &str,
//     // ) -> Result<String, ElementFinderError> {
//     //     self.with_element(selector, class_name, |_, el| {
//     //         el.value().attr(attr_name).map(String::from).ok_or(
//     //             ElementFinderError::AttrNotFound {
//     //                 selector: selector.to_string(), 
//     //                 class_name: class_name.to_string(), 
//     //                 attr_name: attr_name.to_string() 
//     //             })
//     //     })?
//     // }

//     // pub fn element<'a>(&self) -> ElementRef<'a> {

//     // }

//     /// Picks an element that has the given class. Changes the stage to Element.
//     /// 
//     /// Needs stage to be a Select iterator, so panics if stage was Html or Element.
//     /// 
//     /// In other words, should only be called after calling select.
//     pub fn pick_by_class_name<V>(
//         &mut self,
//         selector: &str,
//         class_name: &str,
//         move_stage: bool,
//         and: impl Fn(&ElementRef<'a>) -> V,
//     ) -> Result<V, ElementFinderError> {
//         self.pick_by(
//             selector, 
//             class_name, 
//             |el| el.value().has_class(class_name, CaseSensitivity::AsciiCaseInsensitive),
//             move_stage, 
//             |el| and(el)
//         )
//     }

//     pub fn pick_by<V>(
//         &mut self,
//         selector: &str,
//         class_name: &str,
//         by: impl Fn(&ElementRef<'a>) -> bool,
//         move_stage: bool,
//         and: impl Fn(&ElementRef<'a>) -> V,
//     ) -> Result<V, ElementFinderError> {
//         match self.stage {
//             ThisStage::Html => unreachable!(),
//             ThisStage::SelectInHtml(mut sel) => {
//                 let mut sel = sel.take().unwrap();
//                 let el = sel
//                     .find(|el| by(el))
//                     .ok_or_else(|| ElementFinderError::ElementNotFound {
//                         selector: selector.to_string(), class_name: class_name.to_string()
//                     })?;
//                 let res = and(&el);
//                 self.stage = match move_stage {
//                     true => ThisStage::Element(el),
//                     false => ThisStage::SelectInHtml(Some(sel)),
//                 };
//                 Ok(res)
//             }
//             ThisStage::Element(_) => unreachable!(),
//             ThisStage::SelectInElement(mut sel) => {
//                 let mut sel = sel.take().unwrap();
//                 let el = sel
//                     .find(|el| by(el))
//                     .ok_or_else(|| ElementFinderError::ElementNotFound {
//                         selector: selector.to_string(), class_name: class_name.to_string()
//                     })?;
//                 let res = and(&el);
//                 self.stage = match move_stage {
//                     true => ThisStage::Element(el),
//                     false => ThisStage::SelectInElement(Some(sel)),
//                 };
//                 Ok(res)
//             }
//         }
//     }

//     /// Selects elements that match the given selector. Changes the stage to a Select.
//     /// 
//     /// Needs stage to be either Html or Element, so panics if stage is a Select iterator.
//     pub fn select_and(
//         &mut self,
//         selector: String,
//         class_name: &str,
//         move_stage: bool,
//     ) {
//             // Create a Selector to find the "a" elements
//         let selector_obj = Selector::parse(&selector).unwrap();

//         match self.stage {
//             ThisStage::Html => {
//                 let select = self.html.select(&selector_obj);
//                 self.stage = match move_stage {
//                     true => ThisStage::SelectInHtml(Some(select)),
//                     false => ThisStage::Html,
//                 };
//             },
//             ThisStage::SelectInHtml(_) => unreachable!(),
//             ThisStage::Element(el) => {
//                 let select = el.select(&selector_obj);
//                 self.stage = match move_stage {
//                     true => ThisStage::SelectInElement(Some(select)),
//                     false => ThisStage::Element(el),
//                 };
//             },
//             ThisStage::SelectInElement(_) => unreachable!(),
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub enum ElementFinderError {
//     ElementNotFound {
//         selector: String,
//         class_name: String,
//     },
//     AttrNotFound {
//         selector: String,
//         class_name: String,
//         attr_name: String,
//     },
// }

// pub struct ElementInHtml<'a> {
//     /// We keep a borrowed reference to the html body as ElementRef's are borrowed and the
//     html_body: &'a str,
// }

// impl ElementInHtml {
//     /// Gets the attribute of the matching element in the HTML string.
//     pub fn get_attr(
//         html_body: &str,
//         selector: &str,
//         class_name: &str,
//         attr_name: &str,
//     ) -> Result<String, ElementInHtmlError> {
//         Self::with_element(html_body, selector, class_name, |el| {
//             el.value().attr(attr_name).map(String::from).ok_or(
//                 ElementInHtmlError::AttrNotFound {
//                     selector: selector.to_string(), 
//                     class_name: class_name.to_string(), 
//                     attr_name: attr_name.to_string() 
//                 })
//         })?
//     }

//     /// Provides an iterator over the child elements after finding the element.
//     pub fn with_child_elements<'a, 'b, V>(
//         html_body: &str,
//         selector: &str,
//         class_name: &str,
//         child_selector: &str,
//         with_child_elements: impl Fn(Select<'a, 'b>) -> V,
//     ) -> Result<V, ElementInHtmlError> {
//         Self::with_element(html_body, selector, class_name, |el| {
//             let child_selector = Selector::parse(child_selector).unwrap();
//             let select_iterator = el.select(&child_selector);
//             with_child_elements(select_iterator)
//         })
//     }

//     /// Provides the element that matches has the given selector and class name.
//     /// 
//     /// Returns None if no matching element is found.
//     pub fn with_element<'a, V>(
//         html_body: &str,
//         selector: &str,
//         class_name: &str,
//         with_element: impl Fn(ElementRef<'a>) -> V,
//     ) -> Result<V, ElementInHtmlError> {
//         let html = Html::parse_document(&html_body);
    
//         // Create a Selector to find the "a" elements
//         let selector_obj = Selector::parse(selector).unwrap();
    
//         // Iterate over elements matching the selector
//         for element in html.select(&selector_obj) {
    
//             // Try to get the "href" attribute if this element has the following class
//             if element.value().has_class(class_name, CaseSensitivity::AsciiCaseInsensitive) {
    
//                 return Ok(with_element(element));
//             }
//         }
//         Err(ElementInHtmlError::ElementNotFound {
//             selector: selector.to_string(), class_name: class_name.to_string()
//         })
//     }
// }

// #[derive(Clone, Debug)]
// pub enum ElementInHtmlError {
//     ElementNotFound {
//         selector: String,
//         class_name: String,
//     },
//     AttrNotFound {
//         selector: String,
//         class_name: String,
//         attr_name: String,
//     },
// }
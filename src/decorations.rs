// account for border width
// iterate and == diff on the decoration side

// XWindows struct .update()

use std::collections::HashMap;

use gtk::{Window, WindowType, WidgetExt};
use gdk::WindowExt;

use std::cell::RefCell;
use std::rc::Rc;

use crate::wm;
use crate::wm::events::{Event, EventId, EventValue};
use crate::wm::xcb::xwindows::XWindowData;
use crate::wm::gtk::gdk_get_xid;

use gtk::prelude::*;

pub fn load_decorations(wm_util: &wm::WMUtil) {

    let mut gtk_windows: Rc<RefCell<HashMap<xcb::Window, gtk::Window>>> =
        Rc::new(RefCell::new(HashMap::new()));

    let event_id = wm_util.add_listener(Event::Windows,
        clone!((gtk_windows, wm_util) move |windows_opt| {
            if let Some(EventValue::Windows(event_windows)) = windows_opt {
                for (xwindow, xwindowdata) in event_windows {
                    let XWindowData { x, y, width, height, name, visible } = xwindowdata;
                    let has_existing = {
                        if let Some(window) = gtk_windows.borrow().get(&xwindow) {

                            window.set_title(&name);
                            window.move_(x as i32 - 20, y as i32 - 20);
                            window.resize(width as i32 + 40, height as i32 + 40);

                            if visible {
                                window.show();
                            } else {
                                window.hide();
                            }

                            if let Some(window) = window.get_window() {
                                let id = gdk_get_xid(&window);

                                match xcb::Connection::connect(None) {
                                    Ok((conn, screen_num)) => {
                                        let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
                                        xcb::configure_window(
                                            &conn,
                                            id,
                                            &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW)],
                                        );
                                        conn.flush();
                                    }
                                    _ => {},
                                }
                            }

                            true
                        } else {
                            false
                        }
                    };
                    if !has_existing {
                        // new window
                        let window = gtk::Window::new(WindowType::Popup);
                        wm_util.add_gtk_window(&window);

                        if let Some(window) = window.get_window() {
                            window.set_override_redirect(true);
                            window.set_shadow_width(0, 0, 0, 0);

                            let id = gdk_get_xid(&window);

                            match xcb::Connection::connect(None) {
                                Ok((conn, screen_num)) => {
                                    let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();
                                    xcb::configure_window(
                                        &conn,
                                        id,
                                        &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW)],
                                    );
                                    conn.flush();
                                }
                                _ => {},
                            }
                        }

                        let label = gtk::Label::new(None);

                        label.set_text(&name);
                        label.show();

                        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                        container.add(&label);
                        window.add(&container);
                        WidgetExt::set_name(&container, "info");

                        WidgetExt::set_name(&window, "info");

                        window.set_decorated(false);
                        wm::gtk::set_transparent(&window);
                        window.show_all();

                        window.set_title(&name);
                        window.move_(x as i32 + 20, y as i32 + 20);
                        window.resize(width as i32 -40, height as i32 - 40);

                        if !visible {
                            window.hide();
                        }

                        gtk_windows.borrow_mut().insert(xwindow, window);
                    }
                }

                // remove windows

            }
        })
    );


    // ----

    // let mut windows: HashMap<xcb::Window, XWindowData> = HashMap::new();
    // let mut gtk_windows: HashMap<xcb::Window, gtk::Window> = HashMap::new();

    // let window = gtk::Window::new(WindowType::Popup);
    // wm_util.add_gtk_window(&window);

    //     let label = gtk::Label::new(None);

    //     label.set_text("custom window decoration WIP");
    //     label.show();

    //     let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    //     container.add(&label);
    //     window.add(&container);
    //     WidgetExt::set_name(&container, "info");

    // WidgetExt::set_name(&window, "info");

    // // window.set_type_hint(gdk::WindowTypeHint::Dock);
    // window.set_title("decorations");
    // // window.set_keep_below(true);

    // // window.set_skip_pager_hint(false);
    // // window.set_skip_taskbar_hint(false);
    // // window.set_o
    // window.set_decorated(false);
    // window.move_(100, 900);
    // window.resize(700, 500);
    // // window.stick();

    // wm::gtk::set_transparent(&window);
    // window.show_all();

    // if let Some(window) = window.get_window() {
    //     window.set_override_redirect(true);
    //     // window.set_pass_through(true);
    //     // window.set_static_gravities(true);
    //     // window.set_keep_below(true);
    //     // window.set_modal_hint(true);
    //     window.set_shadow_width(0, 0, 0, 0);
    //     // window.show();
    //     // window.show_unraised();


    //     let id = {
    //         let id = gdk_get_xid(&window);
    //         // println!("{:#?}", id);
    //         id
    //     };

    //     // wait a tick before restacking window

    //     gtk::idle_add(move || {
    //         match xcb::Connection::connect(None) {
    //             Ok((conn, screen_num)) => {
    //                 let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();

    //             // let vim = 37748739;
    //                 // let parent = xcb::query_tree(&conn, id).get_reply().unwrap().parent();
    //                 // xcb::configure_window(
    //                 //     &conn,
    //                 //     parent,
    //                 //     &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW)],
    //                 // );
    //                 // xcb::configure_window(

    //                 //     &conn,
    //                 //     vim,
    //                 //     &[
    //                 //     // (xcb::CONFIG_WINDOW_SIBLING as u16, id),
    //                 //     (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE),
    //                 //     ],
    //                 // );
    //                 xcb::configure_window(
    //                     &conn,
    //                     id,
    //                     &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW)],
    //                 );
    //                 conn.flush();
    //             }
    //             _ => {},
    //         }
    //         gtk::Continue(false)
    //     });
    // }

// // 19:48 <+bdelloid> it's plausible gtk is using multiple virtual windows to draw their shit
// // 19:48 <+bdelloid> I think I had that issue before
// // 19:48 <+bdelloid> you need to find the real windos
// // 19:48 <+bdelloid> window
// // 19:48 <+bdelloid> up the chain

// // unmap / map vim window
// // create an xwindow to put all gtk windows behind

    // let event_id = wm_util.add_listener(Event::Windows,
    //     clone!(window move |windows_opt| {
    //         if let Some(EventValue::Windows(event_windows)) = windows_opt {
    //             let vim = 37748739;
    //             // println!("{:#?}", event_windows);
    //             if let Some(v) = event_windows.get(&vim) {
    //                 window.move_(v.x as i32 - 20 , v.y as i32 - 20);
    //             }

    //             // set behind
    //             if let Some(window) = window.get_window() {


    //                         // xcb::circulate_window(
    //                         //     &conn,
    //                         //     1,
    //                         //     id,
    //                         // );



    //     let id = {
    //         let id = gdk_get_xid(&window);
    //         // println!("{:#?}", id);
    //         id
    //     };

    //     match xcb::Connection::connect(None) {
    //         Ok((conn, screen_num)) => {


    //             let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();

    //             // xcb::unmap_window(&conn, vim);
    //             // conn.flush();


    //             xcb::configure_window(
    //                 &conn,
    //                 id,
    //                 &[
    //                 // (xcb::CONFIG_WINDOW_SIBLING as u16, screen.root()),
    //                 (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW),
    //                 ],
    //             );

    //             // xcb::configure_window(

    //             //     &conn,
    //             //     vim,
    //             //     &[
    //             //     (xcb::CONFIG_WINDOW_SIBLING as u16, id),
    //             //     (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE),
    //             //     ],
    //             // );
    //             // println!("{:#?}", id);

    //                         // xcb::circulate_window(
    //                         //     &conn,
    //                         //     1,
    //                         //     vim,
    //                         // );

    //             // xcb::change_property(
    //             //     &conn,
    //             //     xcb::PROP_MODE_REPLACE as u8,
    //             //     vim,
    //             //     xcb::ATOM_WM_TRANSIENT_FOR,
    //             //     xcb::ATOM_WINDOW,
    //             //     32,
    //             //     &[id],
    //             // );

    //                         // xcb::circulate_window(
    //                         //     &conn,
    //                         //     1,
    //                         //     id,
    //                         // );

    //             // xcb::map_window(&conn, vim);
    //             conn.flush();
    //         }
    //         _ => {},
    //     }

    //            }
    //        }
    //    })
    //);


}

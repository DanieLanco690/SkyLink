use std::cell::RefCell;
use std::rc::Rc;
use crate::base_sim_app_gio::MyApp;
use crate::test::test_bench::*;
use crate::initializer::initialize;
use crate::sim_control::SimulationControl;

mod sim_app;
mod sim_control;
mod initializer;
mod skylink_drone;
mod test;
mod base_sim_app_gio;


fn main() {
    // println!("Hello, world!");


    // Put this to true if you want to use tests
    // or to false if you want to use the Sim Contr application.
    let test = true;
    if test {
        //Comment functions we aren't testing

        // test_generic_fragment_forward();
        // test_generic_drop();
        // test_generic_nack();
        // test_flood();
        // test_double_chain_flood();
        // test_star_flood();
        // test_butterfly_flood();
        // test_tree_flood();
        // test_drone_commands();
        // test_busy_network();
        let (sim_contr, handles) = initialize("inputs/input_star.toml");
        let mut pass = Rc::new(RefCell::new(sim_contr));
        run_sim_gio(pass).expect("TODO: panic message");

    } else {
        let (sim_contr, handles) = initialize("inputs/input_generic_fragment_forward.toml");
        let mut pass = Rc::new(RefCell::new(sim_contr));
        pass.borrow_mut().crash_drone(2);
        sim_app::run_simulation_gui(pass.clone());

        for handle in handles.into_iter() {
            handle.join().unwrap();
        }
    }
}

fn run_sim_gio(sim_control: Rc<RefCell<SimulationControl>>) -> Result<(), eframe::Error>{
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Interfaccia con layout adattabile",
        options,
        Box::new(|_cc| Box::new(MyApp::new(sim_control))),
    )
}

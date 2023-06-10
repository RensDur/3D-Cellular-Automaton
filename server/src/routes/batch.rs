use actix_web::{post, web, Responder, Result};
use serde::{Serialize, Deserialize};
use std::{sync::Mutex, time::Instant, fs::File};

use crate::{CAAppData, appdata::dim3d::automata::{automaton_gpu_n_chemicals::GPUNChemicalsCellularAutomaton3D, automaton::{CellularAutomaton3D, self}}};

use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct BatchEntry {
    species: usize,
    chemical: String,
    variable: String,

    min: f32,
    max: f32,
    step: f32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BatchExportEntry {
    attribute: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BatchExperiment {
    entries: Vec<BatchEntry>,
    export_entries: Vec<BatchExportEntry>,
    iterations: usize,
    file_name: String,
    floating_point: String
}






fn run_experiment(automaton: &mut GPUNChemicalsCellularAutomaton3D, experiment: &BatchExperiment, file: &mut File) {

    // Base case: there's no more variables to vary
    if experiment.entries.len() == 0 {

        // Start by spreading chemicals randomly
        automaton.spread_chemicals_randomly(automaton.chemicals.len() as u32 + 1);

        // Start recording time
        let start = Instant::now();

        // Simply run the specified number of iterations
        for _ in 0..experiment.iterations {
            automaton.run_iteration();
        }

        // Record the simulation time
        let duration = start.elapsed();

        // Write the desired results to the file
        write_results(automaton, experiment, duration.as_secs_f32(), file);

    }


    // Recursive case: there's still variables to vary
    else {

        // Pull the next entry from the experiment and create a new experiment
        // that does not have this entry and is suited for a recursion-call
        let varying = experiment.entries[0].clone();
        let recursive_experiment = BatchExperiment {
            entries: experiment.entries[1..].to_vec(),
            export_entries: experiment.export_entries.clone(),
            iterations: experiment.iterations,
            file_name: experiment.file_name.clone(),
            floating_point: experiment.floating_point.clone()
        };

        // We're going to work with a while loop.
        // Start by registering the start-value from the 'varying' entry
        let mut val = varying.min;

        println!("Varying between {} and {} with steps {}", varying.min, varying.max, varying.step);

        // Continue iterating while 'val' is smaller than or equal to 'max'
        while val <= varying.max {

            // 1. Update the species configuration, by applying this 'val' to the right chemical
            
            if varying.chemical == "promotor" && varying.variable == "range" {

                // Update the range of the promotor of the specified species
                automaton.chemicals[varying.species].promote.range = val;

            } else if varying.chemical == "promotor" && varying.variable == "influence" {

                // Update the influence of the promotor of the specified species
                automaton.chemicals[varying.species].promote.influence = val;

            } else if varying.chemical == "demotor" && varying.variable == "range" {

                // Update the range of the demotor of the specified species
                automaton.chemicals[varying.species].demote.range = val;

            } else if varying.chemical == "demotor" && varying.variable == "influence" {

                // Update the influence of the demotor of the specified species
                automaton.chemicals[varying.species].demote.influence = val;

            } else {
                // An invalid experiment was requested, panic to make the recursion stop
                panic!("Invalid experiment was run: species {}, chemical {}, variable {}", varying.species, varying.chemical, varying.variable);
            }


            // 2. Make a recursive call to this method to possibly vary other variables and run the experiment
            run_experiment(automaton, &recursive_experiment, file);



            // Increase the value with 'step' for the next iteration
            val += varying.step;

        }


        

    }

}


fn write_results(automaton: &GPUNChemicalsCellularAutomaton3D, experiment: &BatchExperiment, sim_time: f32, file: &mut File) {

    let mut line: String = String::from("");

    // Loop over the export entries one by one
    for export_entry in &experiment.export_entries {

        if export_entry.attribute == "number-of-species" {

            // Insert the number of species + 1 for the undifferentiated cell-type
            line.push_str((automaton.chemicals.len()).to_string().as_str());
            line.push(';');

        } else if export_entry.attribute == "chem-values" {

            // For each chemical in the simulation
            for group in &automaton.chemicals {

                // First promotor, then demotor
                // First range, then influence
                line.push_str(group.promote.range.to_string().as_str());
                line.push(';');

                line.push_str(group.promote.influence.to_string().as_str());
                line.push(';');


                line.push_str(group.demote.range.to_string().as_str());
                line.push(';');

                line.push_str(group.demote.influence.to_string().as_str());
                line.push(';');

            }

        } else if export_entry.attribute == "order-parameter" {

            // Insert only the last value of the order parameter
            let op = automaton.get_order_parameters();

            line.push_str(op[op.len()-1].to_string().as_str());
            line.push(';');

        } else if export_entry.attribute == "order-parameter-evolution" {

            // Insert only the last value of the order parameter
            let op = automaton.get_order_parameters();

            for i in op {
                line.push_str(i.to_string().as_str());
                line.push(';');
            }

        } else if export_entry.attribute == "iterations" {

            // Insert the number of iterations that the automaton has performed
            line.push_str(automaton.get_iteration_count().to_string().as_str());
            line.push(';');

        } else if export_entry.attribute == "simulation-time" {

            // The simulation time is given as a parameter to this function
            line.push_str(sim_time.to_string().as_str());
            line.push(';');

        }

    }

    // End the csv-line
    line.push_str("\r\n");

    // If the csv setting for floating-points was set to 'comma', replace all dots with commas
    line = line.replace(".", ",");

    // Write the line to the file
    match file.write_all(line.as_bytes()) {
        Err(e) => panic!("Error when writing to file {}: {}", experiment.file_name, e),
        _ => {}
    }

}




fn write_types(automaton: &GPUNChemicalsCellularAutomaton3D, experiment: &BatchExperiment, file: &mut File) {

    let mut line: String = String::from("");

    // Loop over the export entries one by one
    for export_entry in &experiment.export_entries {

        if export_entry.attribute == "number-of-species" {

            // Insert the number of species + 1 for the undifferentiated cell-type
            line.push_str("Number or species");
            line.push(';');

        } else if export_entry.attribute == "chem-values" {

            // For each chemical in the simulation
            for i in 0..automaton.chemicals.len() {

                let group = &automaton.chemicals[i];

                // First promotor, then demotor
                // First range, then influence
                line.push_str("S");
                line.push_str(i.to_string().as_str());
                
                line.push_str(" Promotor Range");
                line.push(';');

                line.push_str("S");
                line.push_str(i.to_string().as_str());
                
                line.push_str(" Promotor Influence");
                line.push(';');


                line.push_str("S");
                line.push_str(i.to_string().as_str());
                
                line.push_str(" Demotor Range");
                line.push(';');

                line.push_str("S");
                line.push_str(i.to_string().as_str());
                
                line.push_str(" Demotor Influence");
                line.push(';');

            }

        } else if export_entry.attribute == "order-parameter" {

            // Insert only the last value of the order parameter
            line.push_str("Order parameter");
            line.push(';');

        } else if export_entry.attribute == "order-parameter-evolution" {

            // Insert only the last value of the order parameter
            for i in 0..experiment.iterations {
                line.push_str("OP iter ");
                line.push_str(i.to_string().as_str());
                line.push(';');
            }

        } else if export_entry.attribute == "iterations" {

            // Insert the number of iterations that the automaton has performed
            line.push_str("Number of iterations");
            line.push(';');

        } else if export_entry.attribute == "simulation-time" {

            // The simulation time is given as a parameter to this function
            line.push_str("Simulation time");
            line.push(';');

        }

    }

    // End the csv-line
    line.push_str("\r\n");

    // Write the line to the file
    match file.write_all(line.as_bytes()) {
        Err(e) => panic!("Error when writing to file {}: {}", experiment.file_name, e),
        _ => {}
    }

}





#[post("/batch/run-experiment")]
async fn batch_run_experiment(state: web::Data<Mutex<CAAppData>>, experiment: web::Json<BatchExperiment>) -> Result<impl Responder> {

    println!("Running batch experiment");

    // During the entire time-span of this experiment, the server state will be locked.
    // This is done to prevent any other interaction with the server interfere with this experiment.

    let mut state_mod = state.lock().unwrap();


    // Create the specified file
    let mut file_name = String::from(&experiment.file_name);
    file_name.push_str(".csv");

    let path = Path::new(&file_name);

    let mut file = match File::create(&path) {
        Err(e) => panic!("Error when creating file {}: {}", file_name, e),
        Ok(file) => file
    };

    // The first row of the file will indicate the type of values in the column
    write_types(&state_mod.nchem_ca, &experiment, &mut file);

    // Run the batch
    run_experiment(&mut state_mod.nchem_ca, &experiment, &mut file);


    // Drop the lock on the state
    drop(state_mod);

    Ok("")

}
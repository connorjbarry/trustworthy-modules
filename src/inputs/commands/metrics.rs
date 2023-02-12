use std::fs;
use std::io::{prelude::*, Error};
use serde::Deserialize;

static ABSOLUTE_JSON_PATH: &'static str = "temp_metric.json";
#[derive(Deserialize, Debug)]
pub struct Metrics {
    pub ramp_up:f64,
    pub correctness:f64,
    pub bus_factor:f64,
    pub responsiveness:f64,
    pub license:f64,
    pub total:f64,
}


impl Metrics {
    pub fn new() -> Self {
        Metrics {
            ramp_up: -1.0,
            correctness: -1.0,
            bus_factor: -1.0,
            responsiveness: -1.0,
            license: -1.0,
            total: -1.0,
        }
    }


    /* 
        Function: get_ramp_up
        Arguments: moudule_url - the name of the module the metric is graded for
        Return: f64 - between 0 and 1

        Description: This function runs a script and returns the ramp up time metric
        based on !{}! aspects of the module

        Example: 
            let metrics = Metrics::new();
            metrics.get_ramp_up();
    */

    pub fn get_ramp_up(&mut self, _module_url: &str) -> Result<(), Error> {
        //call python script for ramp up
        let mut contents = String::new();
        let mut file = fs::File::open(ABSOLUTE_JSON_PATH).expect("Unable to open file");
        file.read_to_string(&mut contents).expect("Unable to read file");

        let json: serde_json::Value = serde_json::from_str(&contents)?;

        for (key, value) in json.as_object().unwrap().iter() {
            if key == _module_url {
                let ramp_up = value["RampUp"].as_f64().unwrap();
                self.ramp_up = ramp_up;
            }
        }

        Ok(())


        // delete the folder created by the clone
    }

    /* 
        Function: get_correctness
        Arguments: module_url - the name of the module the metric is graded for
        Return: f64 - between 0 and 1

        Description: This function runs a script and returns the correctness metric
        based on !{}! aspects of the module

        Example: 
            let metrics = Metrics::new();
            metrics.get_correctness();
    */

    pub fn get_correctness(&mut self,  _module_url: &str) -> Result<(), Error> {
        let mut contents = String::new();
        let mut file = fs::File::open(ABSOLUTE_JSON_PATH).expect("Unable to open file");
        file.read_to_string(&mut contents).expect("Unable to read file");

        let json: serde_json::Value = serde_json::from_str(&contents)?;

        for (key, value) in json.as_object().unwrap().iter() {
            if key == _module_url {
                let correctness = value["Correctness"].as_f64().unwrap();
                self.correctness = correctness;
            }
        }

        Ok(())
    }


    /* 
        Function: get_bus_factor
        Arguments: module_url - the name of the module the metric is graded for
        Return: f64 - between 0 and 1

        Description: This function runs a script and returns the bus factor metric
        based on !{}! aspects of the module

        Example: 
            let metrics = Metrics::new();
            metrics.get_bus_factor();
    */
    
    pub fn get_bus_factor(&mut self,  _module_url: &str) -> Result<(), Error> {
        let mut contents = String::new();
        let mut file = fs::File::open(ABSOLUTE_JSON_PATH).expect("Unable to open file");
        file.read_to_string(&mut contents).expect("Unable to read file");

        let json: serde_json::Value = serde_json::from_str(&contents)?;

        for (key, value) in json.as_object().unwrap().iter() {
            if key == _module_url {
                let bus_factor = value["BusFactor"].as_f64().unwrap();
                self.bus_factor = bus_factor;
            }
        }

        Ok(())
        }


    /* 
        Function: get_responsiveness
        Arguments: module_url - the name of the module the metric is graded for
        Return: f64 - between 0 and 1

        Description: This function runs a script and returns the responsiveness metric
        based on !{}! aspects of the module

        Example: 
            let metrics = Metrics::new();
            metrics.get_responsiveness();
    */

    pub fn get_responsiveness(&mut self,  _module_url: &str) -> Result<(), Error> {
        let mut contents = String::new();
        let mut file = fs::File::open(ABSOLUTE_JSON_PATH).expect("Unable to open file");
        file.read_to_string(&mut contents).expect("Unable to read file");

        let json: serde_json::Value = serde_json::from_str(&contents)?;

        for (key, value) in json.as_object().unwrap().iter() {
            if key == _module_url {
                let responsiveness = value["ResponsiveMaintainer"].as_f64().unwrap();
                self.responsiveness = responsiveness;
            }
        }

        Ok(()) 
       }


    /* 
        Function: get_license
        Arguments: module_url - the name of the module the metric is graded for
        Return: f64 - between 0 and 1

        Description: This function runs a script and returns the license metric
        based on !{}! aspects of the module

        Example: 
            let metrics = Metrics::new();
            metrics.get_license();
    */


    pub fn get_license(&mut self,  _module_url: &str) -> Result<(), Error> {
        let mut contents = String::new();
        let mut file = fs::File::open(ABSOLUTE_JSON_PATH).expect("Unable to open file");
        file.read_to_string(&mut contents).expect("Unable to read file");

        let json: serde_json::Value = serde_json::from_str(&contents)?;

        for (key, value) in json.as_object().unwrap().iter() {
            if key == _module_url {
                let license = value["License"].as_f64().unwrap();
                self.license = license;
            }
        }

        Ok(()) 
       }


    /* 
        Function: get_total
        Arguments: module_url - the name of the module the metric is graded for
        Return: f64 - between 0 and 1

        Description: This function runs an algorithm considering all the metrics calculated above and returns the total grade

        Example: 
            let metrics = Metrics::new();
            metrics.get_total();
    */

    pub fn get_total(&mut self,  module_url: &str, _api_url: &str) {
        self.get_ramp_up(module_url).expect("Unable to get ramp up");
        self.get_correctness(module_url).expect("Unable to get correctness");
        self.get_bus_factor(module_url).expect("Unable to get bus factor");
        self.get_responsiveness(module_url).expect("Unable to get responsiveness");
        self.get_license(module_url).expect("Unable to get license");

        
        self.total = ((self.ramp_up * 0.25) + (self.correctness * 0.2) + (self.bus_factor * 0.15) + (self.responsiveness * 0.4)) * self.license;
    }  

    /* 
        Function: get_metrics
        Arguments: module_url - the name of the module the metric is graded for
        Return: None

        Description: This function runs all the scripts and prints the metrics for the module in a tabular format

        Example: 
            let metrics = Metrics::new();
            metrics.get_metrics();
    */

    pub fn get_metrics(&mut self, module_url: &str, api_url: &str) {
        self.get_total(module_url, api_url);
        println!("{{\"URL\": \"{}\", \"NET_SCORE\": {:.2}, \"RAMP_UP_SCORE\": {}, \"CORRECTNESS_SCORE\": {}, \"BUS_FACTOR_SCORE\": {}, \"RESPONSIVE_MAINTAINER_SCORE\": {}, \"LICENSE_SCORE\": {}}}",
        module_url, self.total, self.ramp_up, self.correctness, self.bus_factor, self.responsiveness, self.license
        );
    }
}
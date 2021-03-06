
extern crate rand;
extern crate uuid;

use std::collections::HashMap;
use rand::Rng;
use uuid::Uuid;

use super::activation::Activation;
use super::neurontype::NeuronType;



/// Neuron is a wrapper around a neuron providing only what is needed for a neuron to be added 
/// to the NEAT graph, while the neuron encapsulates the neural network logic for the specific nodetype,
/// Some neurons like an LSTM require more variables and different interal activation logic, 
/// so encapsulating that within a normal node on the graph would be misplaced.
#[derive(Deserialize, Serialize, Debug)]
pub struct Neuron {
    pub innov: Uuid,
    pub outgoing: Vec<Uuid>,
    pub incoming: HashMap<Uuid, Option<f32>>,
    pub bias: f32,
    pub value: f32,
    pub d_value: f32,
    pub state: f32,
    pub error: f32,
    pub activation: Activation,
    pub neuron_type: NeuronType
}



impl Neuron {


    pub fn new(innov: Uuid, neuron_type: NeuronType, activation: Activation) -> Self {
        Neuron {
            innov,
            outgoing: Vec::new(),
            incoming: HashMap::new(),
            bias: rand::thread_rng().gen::<f32>(),
            value: 0.0,
            d_value: 0.0,
            state: 0.0,
            error: 0.0,
            activation,
            neuron_type,
        }
    }



    /// Return this struct as a raw mutable pointer - consumes the struct
    pub fn as_mut_ptr(self) -> *mut Neuron {
        Box::into_raw(Box::new(self))
    }



    /// figure out if this node can be calculated, meaning all of the 
    /// nodes pointing to it have given this node their output values.
    /// If they have, this node is ready to be activated
    #[inline]
    pub fn is_ready(&mut self) -> bool {
        self.incoming
            .values()
            .all(|x| x.is_some())
    }


    
    /// 𝜎(Σ(w * i) + b)
    /// activate this node by calling the underlying neuron's logic for activation
    /// given the hashmap of <incoming edge innov, Option<incoming Neuron output value>>
    #[inline]
    pub fn activate(&mut self) {
        self.state = self.incoming
            .values()
            .fold(self.bias, |sum, curr| {
                match curr {
                    Some(x) => sum + x,
                    None => panic!("Cannot activate node.")
                }
            });
        if self.activation != Activation::Softmax {
            self.value = self.activation.activate(self.state);
            self.d_value = self.activation.deactivate(self.state);
        }
    }



    /// deactivate this node by calling the underlying neuron's logic to compute
    /// the gradient of the original output value 
    #[inline]
    pub fn deactivate(&self) -> f32 {
        self.activation.deactivate(self.state)
    }



    /// each Neuron has a base layer of reset which needs to happen 
    /// but on top of that each neuron might need to do more interanally
    #[inline]
    pub fn reset_neuron(&mut self) {
        self.error = 0.0;
        for (_, val) in self.incoming.iter_mut() {
            *val = None;
        }
    }


}



impl Clone for Neuron {
    fn clone(&self) -> Self { 
        Neuron {
            innov: self.innov,
            outgoing: self.outgoing
                .iter()
                .map(|x| *x)
                .collect(),
            incoming: self.incoming
                .iter()
                .map(|(key, _)| (*key, None))
                .collect(),
            state: 0.0,
            value: 0.0,
            d_value: 0.0,
            error: 0.0,
            bias: self.bias.clone(),
            activation: self.activation.clone(),
            neuron_type: self.neuron_type.clone(),
        }
    }
}


impl PartialEq for Neuron {
    fn eq(&self, other: &Self) -> bool {
        self.innov == other.innov
    }
}



use std::collections::HashMap;
use super::layer::Layer;
use super::neurons::{
    neuron::Neuron,
    dense::Dense
};
use super::activation::Activation;
use super::nodetype::NodeType;



/// Vertex is meant to seperate the neuron logic from the node on the NEAT graph,
/// a node on the NEAT graph only needs pointers to the edges around it and 
/// the values being propagated through it. Some neurons like an LSTM require more 
/// variables and different interal activation logic, so encapsulating that within
/// a normal node on the graph would be misplaced. Because of this each vertex holds a 
/// neuron which is a specific type of node which encapsulates the unique logic referenced above
#[derive(Debug)]
pub struct Vertex {
    pub innov: i32,
    pub outgoing: Vec<i32>,
    pub incoming: HashMap<i32, Option<f64>>,
    pub curr_value: Option<f64>,
    pub layer_type: Layer,
    pub neuron: Box<dyn Neuron>
}



impl Vertex {

    /// Return a new vertex 
    pub fn new(innov: i32, layer_type: Layer, node_type: NodeType, activation: Activation) -> Self {
        Vertex {
            innov,
            outgoing: Vec::new(),
            incoming: HashMap::new(),
            curr_value: None,
            layer_type,
            neuron: Vertex::neuron_factory(node_type, activation)
        }
    }



    /// Return this struct as a raw mutable pointer - consumes the struct
    pub fn as_mut_ptr(self) -> *mut Vertex {
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


    
    /// activate this node by calling the underlying neuron's logic for activation
    /// given the hashmap of <incoming edge innov, Option<incoming vertex output value>>
    #[inline]
    pub fn activate(&mut self) {
        self.curr_value = Some(self.neuron.activate(&self.incoming));
    }



    /// deactivate this node by calling the underlying neuron's logic to compute
    /// the gradient of the original output value 
    #[inline]
    pub fn deactivate(&mut self) -> f64 {
        self.neuron.deactivate(self.curr_value.unwrap())
    }



    /// each vertex has a base layer of reset which needs to happen 
    /// but on top of that each neuron might need to do more interanally
    #[inline]
    pub fn reset_neuron(&mut self) {
        self.curr_value = None;
        for (_, val) in self.incoming.iter_mut() {
            *val = None;
        }
        self.neuron.reset();
    }



    /// given a nodetype and activation enum, return a boxed struct which implements the neuruon trait
    #[inline]
    fn neuron_factory(node_type: NodeType, activation: Activation) -> Box<dyn Neuron> {
        match node_type {
            NodeType::Dense => Box::new(Dense { activation }),
            NodeType::LSTM => Box::new(Dense { activation }),
            NodeType::Recurrent => Box::new(Dense { activation })
        }
    }


}


impl Clone for Vertex {
    fn clone(&self) -> Self { 
        Vertex {
            innov: self.innov,
            outgoing: self.outgoing.iter().map(|x| *x).collect(),
            incoming: self.incoming.iter().map(|(key, val)| (*key, *val)).collect(),
            curr_value: self.curr_value.clone(),
            layer_type: self.layer_type.clone(),
            neuron: self.neuron.clone()
        }
    }
}


impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.innov == other.innov
    }
}
pub mod layer;
pub mod dense;


pub mod layertype {

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum LayerType {
        DensePool,
        Dense,
        LSTM, 
        Recurrent
    }

}
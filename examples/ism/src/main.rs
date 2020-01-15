
extern crate radiate;
extern crate csv;


use std::error::Error;
use radiate::prelude::*;


// adam optimizer: https://gluon.mxnet.io/chapter06_optimization/adam-scratch.html


fn main() -> Result<(), Box<dyn Error>> {

    let neat_env = NeatEnvironment::new()
        .set_weight_mutate_rate(0.8)
        .set_edit_weights(0.1)
        .set_weight_perturb(1.5)
        .set_new_node_rate(0.04)
        .set_new_edge_rate(0.04)
        .set_reactivate(0.2)
        .set_activation_functions(vec![
            Activation::Sigmoid,
            Activation::Relu,
        ]);


    let mut net = Neat::new()
        .input_size(3)
        .lstm(3, 1, Activation::Sigmoid);

        
    let ism = ISM::new(3);
    let num_evolve = 30;
    let (mut solution, _) = Population::<Neat, NeatEnvironment, ISM>::new()
        .constrain(neat_env)
        .size(100)
        .populate_clone(net)
        .debug(true)
        .dynamic_distance(true)
        .stagnation(10, vec![Genocide::KillWorst(0.9)])
        .configure(Config {
            inbreed_rate: 0.001,
            crossover_rate: 0.75,
            distance: 0.5,
            species_target: 5
        })
        .run(|_, fit, num| {
            println!("Generation: {} score: {}", num, fit);
            num == num_evolve
        })?;
            
    solution.reset();
    ism.show(&mut solution);

    println!("Training\n\n");
    solution.reset();
    solution.train(&ism.inputs, &ism.answers, 200, 0.005, ism.inputs.len())?;

    solution.reset();
    ism.show(&mut solution);

    solution.reset();
    println!("{:?}", ism.solve(&mut solution));
    solution.reset();
    ism.write_data(&mut solution);
    // ism.freestyle(3, &mut solution); 

    Ok(())
}





#[derive(Debug)]
pub struct ISM {
    min_v: f32,
    max_v: f32,
    inputs: Vec<Vec<f32>>,
    answers: Vec<Vec<f32>>
}



 
impl ISM {

    pub fn new(back: usize) -> Self {
        ISM::read_data(back)
    }
    
    
    fn layer(back: usize, data: Vec<f32>) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
        let mut output = Vec::new();
        let mut answer = Vec::new();
        for i in 0..data.len() - back{
            let mut temp = Vec::with_capacity(back);
            for j in 0..back {
                temp.push(data[i + j]);
            }
            answer.push(vec![data[i + back]]);
            output.push(temp);
        }
        // output.reverse();
        // answer.reverse();
        (output, answer)
    }


    fn minimum(nums: &Vec<Vec<f32>>) -> f32 {
        nums.iter()
            .fold(1000.0, |min, curr| {
                if curr[0] < min {
                    return curr[0]
                }
                min
            })
    }


    fn maximum(nums: &Vec<Vec<f32>>) -> f32 {
        nums.iter()
            .fold(-1000.0, |max, curr| {
                if curr[0] > max {
                    return curr[0]
                }
                max
            })
    }


    fn read_data(back: usize) -> Self {
        let mut reader = csv::Reader::from_path("C:/Users/peter/Desktop/software/radiate/examples/ism/src/ism_input.csv").unwrap();
        let mut data = Vec::new();
        for result in reader.records() {
            let temp = result.unwrap();
            let val = temp.get(1).unwrap().parse().unwrap();
            data.push(vec![val]);
        }
        let smallest = ISM::minimum(&data);
        let biggest = ISM::maximum(&data);
        // let total = data.iter().map(|x| x[0]).collect::<Vec<_>>().iter().sum::<f32>();

        // let mean = total / data.len() as f32;

        // let mut t = data
        //     .iter()
        //     .map(|x| (x[0] - mean).powf(2.0))
        //     .collect::<Vec<_>>()
        //     .iter()
        //     .sum::<f32>();
        // t = (1.0 / data.len() as f32) * t;
        // t = t.sqrt();

        // data = data.iter()
        //     .map(|x| vec![((x[0] - mean) / t)])
        //     .collect::<Vec<_>>();

        // let s = ISM::minimum(&data);
        // data = data.iter()
        //     .map(|x| vec![x[0] + s])
        //     .collect::<Vec<_>>();
        // let smallest = ISM::minimum(&data);
        // let biggest = ISM::maximum(&data);
        data = data.iter()
            .map(|x| {
                vec![(x[0] - smallest) / (biggest - smallest)]
            })
            .collect();
                   
                           
        let mut temp = data.iter().map(|x| x[0]).collect::<Vec<_>>();
        temp.reverse();
        // data.reverse();
        let (o, a) = ISM::layer(back, temp);
        ISM {
            min_v: smallest,
            max_v: biggest,
            inputs: o,
            answers: a
        }
    }



    fn write_data(&self, solution: &mut Neat) {
        let mut writer = csv::Writer::from_path("C:/Users/peter/Desktop/software/radiate/examples/ism/src/ism.csv").unwrap();
        for (i, o) in self.inputs.iter().zip(self.answers.iter()) {
            let guess = solution.forward(i).unwrap();
            writer.write_record(&[i[i.len() - 1].to_string(), o[0].to_string(), guess[0].to_string()]).unwrap();
        }
        writer.flush().unwrap();
    }


    fn de_norm(&self, val: f32) -> f32 {
        val * (self.max_v - self.min_v) + self.min_v
    }


    fn show(&self, model: &mut Neat) {
        println!("\n");
        for (i, o) in self.inputs.iter().zip(self.answers.iter()) {
            let guess = model.forward(i).unwrap();
            println!("Input: {:.2?} Answer: {:.2?} Guess: {:.2?}", i[0], o[0], guess[0]);
        }
    }


    fn freestyle(&self, num: usize, model: &mut Neat) {
        let mut guess = Vec::new();
        for (i, o) in self.inputs.iter().zip(self.answers.iter()) {
            guess = model.forward(i).unwrap();
        }

        let mut temp = guess.clone();
        for _ in 0..num {
            guess = model.forward(&temp.to_vec()).unwrap();
            println!("Free Style: input: {:?}, output: {:?}", temp, guess);
            temp = guess.clone();
        }
    }



}


unsafe impl Send for ISM {}
unsafe impl Sync for ISM {}




impl Problem<Neat> for ISM {

    fn empty() -> Self { ISM::new(3) }

    fn solve(&self, model: &mut Neat) -> f32 {
        let mut total = 0.0;
        // for i in 0..self.inputs.len() - 2 {
        //     match model.forward(&self.inputs[i]) {
        //         Some(guess) => total += (guess[0] - self.inputs[i + 1][0]).powf(2.0),
        //         None => panic!("asdf")
        //     }
        // }
        for (ins, outs) in self.inputs.iter().zip(self.answers.iter()) {
            match model.forward(&ins) {
                Some(guess) => total += (guess[0] - outs[0]).powf(2.0),
                None => panic!("Error in training NEAT")
            }
        }
        1.0 - ((1.0 / (self.answers.len()) as f32) * total)
    }
    
}
extern crate rand;

use rand::Rng;

#[derive(Debug)]
pub struct NeuralMatrix 
{
    pub(crate) matrix: Vec<Vec<f64>>,
    pub(crate) bias: Vec<f64>,
    pub(crate) input_size: usize,
    pub output_size: usize,
}


impl NeuralMatrix 
{    
    // nouvelle génération d'un poid de matrice
    pub fn new(input_size: usize, output_size: usize) -> Self 
    {
        // Initialisation de la matrice avec une capacité max défini
        let mut matrix = Vec::with_capacity(output_size);
        let mut bias = Vec::with_capacity(output_size);
        let mut rng = rand::thread_rng();
        
        for _ in 0..output_size 
        {
            // Initialisation des poids
            let neuron_weights: Vec<f64> = (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
            matrix.push(neuron_weights);

            // Initialisation du biais pour chaque neurone
            bias.push(0.0);
        }

        NeuralMatrix 
        {
            matrix,
            bias,
            input_size,
            output_size,
        }
    }



    // Fonction pour effectuer la propagation avant à travers la couche
    pub fn forward(&self, input: &[f64], is_classification: bool) -> Vec<f64> 
    {

        //vérifie que la taille de l'input soit à la bonne taille
        assert_eq!(input.len(), self.input_size);
        //défini la taille du neuronne = à l'output size
        let mut neuron_outputs = Vec::with_capacity(self.output_size);

        for neuron_weights in &self.matrix 
        {
            let mut neuron_output = 0.0;
            for (input_val, weight) in input.iter().zip(neuron_weights) 
            {
                neuron_output += input_val * weight;
            }
            neuron_outputs.push(neuron_output);
        }

        neuron_outputs.iter_mut().zip(&self.bias).for_each(|(o, b)| *o += *b as f64);

        if is_classification 
        {
            if is_classification && self.output_size > 1 {
                println!("softmax utilisé");
                neuron_outputs = crate::activation::softmax(&neuron_outputs);
            } else if is_classification {
                neuron_outputs.iter_mut().for_each(|o| *o = crate::activation::tanh(*o));
            }
            
        }

        neuron_outputs
    }
    
}
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

    // _________________________________ Initialisation des couches _________________________________
    pub fn new(input_size: usize, output_size: usize) -> Self 
    {
        // Initialisation de la matrice avec une capacité max défini pour éviter la réallocation
        let mut matrix = Vec::with_capacity(output_size); // vecteur de taille output_size
        let mut bias = Vec::with_capacity(output_size); // vecteur de taille output_size
        let mut rng = rand::thread_rng();
        
        for _ in 0..output_size 
        {
            let neuron_weights: Vec<f64> = (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
            matrix.push(neuron_weights);
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



    // _________________________________ Propagation _________________________________
    pub fn forward(&self, input: &[f64], is_classification: bool, is_output_layer: bool, output_size: usize) -> Vec<f64> 
    {

        assert_eq!(input.len(), self.input_size);
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

        //pour chacun des sommes pondéré ajoute le biais 
        neuron_outputs.iter_mut().zip(&self.bias).for_each(|(o, b)| *o += *b as f64);

        if is_classification 
        {
            if output_size > 1 
            {
                neuron_outputs = crate::activation::softmax(&neuron_outputs);
            }else
            {
                neuron_outputs.iter_mut().for_each(|o| *o = crate::activation::tanh(*o)); 
            }
        }
        else if !is_output_layer 
        {
            neuron_outputs.iter_mut().for_each(|o| *o = crate::activation::tanh(*o));
        }

        neuron_outputs
    }
    
}
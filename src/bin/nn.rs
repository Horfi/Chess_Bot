use tch::{nn, nn::Module, nn::OptimizerConfig, Tensor};

#[derive(Debug)]
struct ChessNN {
    model: nn::Sequential,
}

impl ChessNN {
    fn new(vs: &nn::Path) -> Self {
        let model = nn::seq()
            .add(nn::linear(vs / "layer1", 64, 128, Default::default()))  // 64 input features (board positions)
            .add_fn(|xs| xs.relu())
            .add(nn::linear(vs / "layer2", 128, 128, Default::default())) // Hidden layer
            .add_fn(|xs| xs.relu())
            .add(nn::linear(vs / "output", 128, 64, Default::default())); // Output 64 (board position to move)

        ChessNN { model }
    }

    // Forward pass
    fn forward(&self, xs: &Tensor) -> Tensor {
        self.model.forward(xs)
    }
}

fn main() {
    // Device selection: CPU or CUDA
    let device = tch::Device::cuda_if_available();

    // Create the neural network and optimizer
    let vs = nn::VarStore::new(device);
    let net = ChessNN::new(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 1e-3).unwrap();

    // Example: Prepare fake chess data (Replace with actual chess data)
    let board_positions = Tensor::rand(&[1000, 64], (tch::Kind::Float, device)); // 1000 games, 64 features (board state)
    let target_moves = Tensor::rand(&[1000, 64], (tch::Kind::Float, device));    // 1000 games, 64 possible moves

    // Training loop
    for epoch in 1..100 {
        let prediction = net.forward(&board_positions); // Predict the next move
        let loss = prediction.mse_loss(&target_moves, tch::Reduction::Mean); // Use MSE loss

        // Backpropagate
        opt.backward_step(&loss);

        if epoch % 10 == 0 {
            println!("Epoch: {}, Loss: {:?}", epoch, f64::from(loss));
        }
    }

    // Save the model
    vs.save("chess_model.pt").unwrap();
}

// src/network.rs

// P2P Network Node Communication

// The Network struct contains functionalities for handling communication between P2P nodes.
pub struct Network {
    // List of connected peers
    peers: Vec<String>,
}

impl Network {
    // Creates a new Network instance
    pub fn new() -> Self {
        Network { peers: Vec::new() }
    }

    // Adds a new peer to the network
    pub fn add_peer(&mut self, peer: String) {
        self.peers.push(peer);
    }

    // Sends a message to a specific peer
    pub fn send_message(&self, peer: &String, message: &str) {
        // Here you would implement the actual sending logic, e.g., using sockets.
        println!("Sending message to {}: {}", peer, message);
    }

    // Broadcasts a message to all peers in the network
    pub fn broadcast_message(&self, message: &str) {
        for peer in &self.peers {
            self.send_message(peer, message);
        }
    }
}

# ObsuraLink  

ObsuraLink is a privacy-focused network that encrypts internet traffic through multiple nodes, functioning similarly to the Tor network. It ensures anonymity by forwarding traffic through a series of secure nodes.  

## Features  

- **Multi-node encryption**: Traffic is encrypted at multiple layers as it passes through nodes.  
- **Custom proxy service**: A lightweight Rust-based proxy designed for secure forwarding.  
- **Client application**: A Rust + Tauri-based client that connects to the network with minimal setup.  
- **Self-hosted nodes**: Users can run their own nodes for enhanced privacy and control.  

## Technologies Used  

- **Rust** (Core development language)  
- **Tauri** (Client application)  
- **Rocket Framework** (Node service API)  

## Installation  

### Client Application  
1. Download the latest client release from the [Releases](https://github.com/your-repo/releases) page.  
2. Install and run the application. It will automatically connect to the network.  

### Running a Node  
1. Install Rust (`rustup install stable`).  
2. Clone the repository:  

   git clone https://github.com/your-repo/obsuralink.git  
   cd obsuralink/node  

3. Build and run the node service:  

   cargo run --release  

4. Configure the node settings in the provided config file.  

## Development Progress  

- ✅ Forwarding traffic through multiple nodes  
- 🚧 Encrypting traffic between nodes (in progress)  

## Contributing  

This project is maintained by **Anthony Bruno**. Contributions are welcome!  

- **Report Issues**: Open an issue on the [GitHub Issues](https://github.com/your-repo/issues) page.  
- **Submit Fixes**: Fork the repo, make changes, and submit a pull request.  

## License  

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.  

---

*ObsuraLink aims to provide a robust, decentralized privacy solution. Stay tuned for updates!*  
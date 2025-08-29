#!/bin/bash

# Solana Memes Project Setup Script
# This script sets up the development environment for the Solana memecoin project

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check Node.js version
check_node_version() {
    if command_exists node; then
        NODE_VERSION=$(node --version | cut -d'v' -f2 | cut -d'.' -f1)
        if [ "$NODE_VERSION" -lt 18 ]; then
            print_error "Node.js version 18 or higher is required. Current version: $(node --version)"
            exit 1
        fi
        print_success "Node.js version: $(node --version)"
    else
        print_error "Node.js is not installed. Please install Node.js 18 or higher."
        exit 1
    fi
}

# Function to check Rust installation
check_rust() {
    if command_exists rustc; then
        print_success "Rust version: $(rustc --version)"
    else
        print_warning "Rust is not installed. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
        print_success "Rust installed successfully"
    fi
}

# Function to check Solana CLI
check_solana() {
    if command_exists solana; then
        print_success "Solana CLI version: $(solana --version)"
    else
        print_warning "Solana CLI is not installed. Installing Solana CLI..."
        sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"
        print_success "Solana CLI installed successfully"
    fi
}

# Function to check Anchor
check_anchor() {
    if command_exists anchor; then
        print_success "Anchor version: $(anchor --version)"
    else
        print_warning "Anchor is not installed. Installing Anchor..."
        cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
        avm install latest
        avm use latest
        print_success "Anchor installed successfully"
    fi
}

# Function to setup environment file
setup_env() {
    if [ ! -f .env ]; then
        print_status "Creating .env file from template..."
        cp env.example .env
        print_success ".env file created. Please update it with your configuration."
    else
        print_warning ".env file already exists. Skipping..."
    fi
}

# Function to install dependencies
install_dependencies() {
    print_status "Installing Node.js dependencies..."
    npm install
    
    print_status "Installing Rust dependencies..."
    cargo build
    
    print_success "Dependencies installed successfully"
}

# Function to setup database
setup_database() {
    print_status "Setting up database..."
    
    # Check if PostgreSQL is running
    if command_exists pg_isready; then
        if pg_isready -q; then
            print_success "PostgreSQL is running"
        else
            print_warning "PostgreSQL is not running. Please start PostgreSQL service."
        fi
    else
        print_warning "PostgreSQL client not found. Please install PostgreSQL."
    fi
}

# Function to build project
build_project() {
    print_status "Building the project..."
    
    # Build Rust programs
    anchor build
    
    # Build frontend
    npm run build:client
    
    print_success "Project built successfully"
}

# Function to run tests
run_tests() {
    print_status "Running tests..."
    
    # Run Rust tests
    anchor test
    
    # Run frontend tests
    npm test
    
    print_success "Tests completed successfully"
}

# Main setup function
main() {
    print_status "Starting Solana Memes project setup..."
    
    # Check prerequisites
    check_node_version
    check_rust
    check_solana
    check_anchor
    
    # Setup environment
    setup_env
    
    # Install dependencies
    install_dependencies
    
    # Setup database
    setup_database
    
    # Build project
    build_project
    
    # Run tests
    run_tests
    
    print_success "Setup completed successfully!"
    print_status "Next steps:"
    echo "1. Update .env file with your configuration"
    echo "2. Start the development server: npm run dev"
    echo "3. Deploy to localnet: npm run deploy:local"
    echo "4. Visit http://localhost:3000 to see the application"
}

# Run main function
main "$@"

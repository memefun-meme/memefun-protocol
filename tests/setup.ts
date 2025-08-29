// Test setup file for Jest
import '@testing-library/jest-dom';

// Mock Solana web3.js
jest.mock('@solana/web3.js', () => ({
  Connection: jest.fn().mockImplementation(() => ({
    connect: jest.fn(),
    disconnect: jest.fn(),
    getLatestBlockhash: jest.fn().mockResolvedValue({
      blockhash: 'test-blockhash',
      lastValidBlockHeight: 1000,
    }),
  })),
  Keypair: {
    generate: jest.fn().mockReturnValue({
      publicKey: { toBase58: () => 'test-public-key' },
      secretKey: new Uint8Array(64),
    }),
  },
  PublicKey: jest.fn().mockImplementation((key) => ({
    toBase58: () => key || 'test-public-key',
    toString: () => key || 'test-public-key',
  })),
}));

// Mock Anchor
jest.mock('@project-serum/anchor', () => ({
  Program: jest.fn(),
  AnchorProvider: jest.fn().mockImplementation(() => ({
    connection: {
      connect: jest.fn(),
      disconnect: jest.fn(),
    },
    wallet: {
      publicKey: { toBase58: () => 'test-wallet-key' },
      signTransaction: jest.fn(),
    },
  })),
}));

// Mock SPL Token
jest.mock('@solana/spl-token', () => ({
  TOKEN_PROGRAM_ID: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
  getAssociatedTokenAddress: jest.fn().mockResolvedValue('test-token-address'),
  createAssociatedTokenAccountInstruction: jest.fn(),
  createTransferInstruction: jest.fn(),
  createMintToInstruction: jest.fn(),
  createBurnInstruction: jest.fn(),
}));

// Global test utilities
global.console = {
  ...console,
  // Uncomment to suppress console.log during tests
  // log: jest.fn(),
  // debug: jest.fn(),
  // info: jest.fn(),
  // warn: jest.fn(),
  // error: jest.fn(),
};

// Mock fetch for API calls
global.fetch = jest.fn();

// Mock localStorage
const localStorageMock = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn(),
};
global.localStorage = localStorageMock;

// Mock sessionStorage
const sessionStorageMock = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn(),
};
global.sessionStorage = sessionStorageMock;

// Test environment setup
beforeEach(() => {
  jest.clearAllMocks();
});

afterEach(() => {
  jest.clearAllMocks();
});

// Custom matchers for testing
expect.extend({
  toBeValidPublicKey(received) {
    const pass = typeof received === 'string' && received.length === 44;
    if (pass) {
      return {
        message: () => `expected ${received} not to be a valid public key`,
        pass: true,
      };
    } else {
      return {
        message: () => `expected ${received} to be a valid public key`,
        pass: false,
      };
    }
  },
});

// Type declarations for custom matchers
declare global {
  namespace jest {
    interface Matchers<R> {
      toBeValidPublicKey(): R;
    }
  }
}

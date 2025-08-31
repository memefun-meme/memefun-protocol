// Format SOL amount
export function formatSOL(amount, decimals = 4) {
  if (amount === 0) return '0 SOL';
  
  if (amount >= 1000000) {
    return `${(amount / 1000000).toFixed(decimals)}M SOL`;
  } else if (amount >= 1000) {
    return `${(amount / 1000).toFixed(decimals)}K SOL`;
  } else {
    return `${amount.toFixed(decimals)} SOL`;
  }
}

// Format token amount
export function formatToken(amount, symbol = 'TOKEN', decimals = 2) {
  if (amount === 0) return `0 ${symbol}`;
  
  if (amount >= 1000000) {
    return `${(amount / 1000000).toFixed(decimals)}M ${symbol}`;
  } else if (amount >= 1000) {
    return `${(amount / 1000).toFixed(decimals)}K ${symbol}`;
  } else {
    return `${amount.toFixed(decimals)} ${symbol}`;
  }
}

// Format percentage
export function formatPercentage(value, decimals = 2) {
  return `${(value * 100).toFixed(decimals)}%`;
}

// Format date
export function formatDate(timestamp) {
  const date = new Date(timestamp * 1000);
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

// Format time ago
export function formatTimeAgo(timestamp) {
  const now = Date.now() / 1000;
  const diff = now - timestamp;
  
  if (diff < 60) return 'Just now';
  if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
  if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
  if (diff < 2592000) return `${Math.floor(diff / 86400)}d ago`;
  
  return formatDate(timestamp);
}

// Format transaction signature (short)
export function formatSignature(signature, length = 8) {
  if (!signature) return 'N/A';
  return `${signature.slice(0, length)}...${signature.slice(-length)}`;
}

// Format address (short)
export function formatAddress(address, length = 6) {
  if (!address) return 'N/A';
  return `${address.slice(0, length)}...${address.slice(-length)}`;
}

// Format number with commas
export function formatNumber(num) {
  return new Intl.NumberFormat('en-US').format(num);
}

// Format currency
export function formatCurrency(amount, currency = 'USD') {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency,
  }).format(amount);
}

// Get Solscan URL for transaction
export function getSolscanUrl(signature) {
  return `https://solscan.io/tx/${signature}`;
}

// Get Solscan URL for address
export function getSolscanAddressUrl(address) {
  return `https://solscan.io/account/${address}`;
}

// Get Solscan URL for token
export function getSolscanTokenUrl(mint) {
  return `https://solscan.io/token/${mint}`;
}

// Calculate percentage change
export function calculatePercentageChange(oldValue, newValue) {
  if (oldValue === 0) return newValue > 0 ? 100 : 0;
  return ((newValue - oldValue) / oldValue) * 100;
}

// Format file size
export function formatFileSize(bytes) {
  if (bytes === 0) return '0 Bytes';
  
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// Truncate text
export function truncateText(text, maxLength = 50) {
  if (text.length <= maxLength) return text;
  return text.slice(0, maxLength) + '...';
}

// Capitalize first letter
export function capitalize(str) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

// Format duration
export function formatDuration(seconds) {
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ${seconds % 60}s`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ${Math.floor((seconds % 3600) / 60)}m`;
  return `${Math.floor(seconds / 86400)}d ${Math.floor((seconds % 86400) / 3600)}h`;
}

function executeParameterChange(
    string memory param,
    uint256 newValue
) external onlyExecutedProposal {
    if (keccak256(bytes(param)) == keccak256("block_size")) {
        BitcoinParams.setMaxBlockSize(newValue);
    }
    // Additional parameters...
} 
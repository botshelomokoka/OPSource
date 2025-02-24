pragma solidity ^0.8.19;

import "@aragon/osx/core/dao/DAO.sol";

contract AnyaGovernance is DAO {
    struct Proposal {
        bytes32 targetHash;
        uint256 votingPower;
        uint256 endTime;
        bool executed;
    }
    
    mapping(bytes32 => Proposal) public proposals;
    
    uint256 public constant EXECUTION_DELAY = 2 days;
    uint256 public constant VETO_PERIOD = 1 days;
    
    using SafeMath for uint256;
    mapping(address => uint256) private _voteWeight;
    uint256 private _totalVotingPower;
    
    function createProposal(
        bytes32 targetHash, 
        bytes calldata execData
    ) external onlyMember {
        proposals[targetHash] = Proposal({
            targetHash: targetHash,
            votingPower: 0,
            endTime: block.timestamp + 7 days,
            executed: false
        });
    }
    
    function vote(bytes32 proposalId, uint256 weight) external {
        uint256 quadraticWeight = sqrt(weight);
        _voteWeight[msg.sender] = _voteWeight[msg.sender].add(quadraticWeight);
        _totalVotingPower = _totalVotingPower.add(quadraticWeight);
        
        require(_totalVotingPower <= MAX_VOTING_POWER, "Voting cap exceeded");
    }
    
    function executeProposal(bytes32 targetHash) external {
        Proposal storage prop = proposals[targetHash];
        require(block.timestamp > prop.endTime + EXECUTION_DELAY, "Before execution delay");
        require(block.timestamp < prop.endTime + EXECUTION_DELAY + VETO_PERIOD, "Veto period expired");
        
        require(_voteWeight[targetHash] < _totalVotingPower.mul(20).div(100),
            "Veto threshold reached");
        
        require(prop.votingPower > quorum(), "Insufficient votes");
        (bool success, ) = address(this).delegatecall(prop.execData);
        require(success, "Execution failed");
        prop.executed = true;
    }
} 
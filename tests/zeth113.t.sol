pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {AnyVerifier} from "../contracts/anyVerifier.sol";

struct OnChainG16 {
    bytes journal;
    bytes seal;
}

contract Zeth113Test is Test {
    using stdJson for string;

    AnyVerifier public anyVerifier;

    function loadOnChainG16() public view returns (OnChainG16 memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/apps/proofs/on-chain-g16-1_1_3.zkp.json");
        string memory json = vm.readFile(path);
        bytes memory jsonBytes = json.parseRaw(".");
        return abi.decode(jsonBytes, (OnChainG16));
    }

    function setUp() public {
        anyVerifier = new AnyVerifier(0x90b9024466aab0b2fd9a9d3d9f0d4db7d2b0e3e6caf9e28093d96c36be6db809);
    }

    function test_valid_seal() public view {
        OnChainG16 memory on_chain_g16 = loadOnChainG16();

        // console.logBytes(on_chain_g16.seal);

        anyVerifier.verify_proof(on_chain_g16.journal, on_chain_g16.seal);
    }

    function testFail_invalid_seal() public view {
        OnChainG16 memory on_chain_g16 = loadOnChainG16();
        on_chain_g16.seal[5] = 0x49;

        anyVerifier.verify_proof(on_chain_g16.journal, on_chain_g16.seal);
    }
}

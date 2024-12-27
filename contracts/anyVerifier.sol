pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ControlID, RiscZeroGroth16Verifier} from "risc0/groth16/RiscZeroGroth16Verifier.sol";

contract AnyVerifier {
    IRiscZeroVerifier public immutable verifier;
    bytes32 public imageId;

    constructor(bytes32 _imageId) {
        verifier = new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
        imageId = _imageId;
    }

    function verify_proof(bytes calldata journal, bytes calldata seal) public view {
        verifier.verify(seal, imageId, sha256(journal));
    }
}
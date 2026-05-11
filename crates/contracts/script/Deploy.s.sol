// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "forge-std/Script.sol";

import "../src/RollupVerifier.sol";

contract Deploy is Script {

    function run() external {

        /*
            Load deployer private key
            from environment.
        */
        uint256 deployerKey =
            vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(
            deployerKey
        );

        /*
            Initial empty state root.
        */
        bytes32 initialRoot =
            bytes32(0);

        RollupVerifier verifier =
            new RollupVerifier(
                initialRoot
            );

        vm.stopBroadcast();

        console.log(
            "Verifier deployed at:",
            address(verifier)
        );
    }
}
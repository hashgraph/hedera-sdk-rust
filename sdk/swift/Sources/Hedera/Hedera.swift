// Enforce minimum Swift version for all platforms and build systems.
#if swift(<5.3)
    #error("Hedera SDK doesn't support Swift versions below 5.3.")
#endif

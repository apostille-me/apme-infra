# Production environment

This is the orchestration root for production-specific infrastructure not already owned by a provider-native Git integration root.

Keep provider-native roots canonical when an external provider syncs a working directory directly. Production orchestration may compose reusable modules, but it must not create a second owner for the same resource graph.

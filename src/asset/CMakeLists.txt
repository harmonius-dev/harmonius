add_library(harmonius-asset STATIC)
target_sources(harmonius-asset
    PUBLIC FILE_SET CXX_MODULES FILES
        harmonius.asset.cppm
)
target_link_libraries(harmonius-asset
    PUBLIC harmonius-gpu harmonius-rg-core harmonius-rg-exec
)

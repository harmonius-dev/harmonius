add_library(harmonius-rg-diag STATIC)
target_sources(harmonius-rg-diag
    PUBLIC FILE_SET CXX_MODULES FILES
        harmonius.rg.diag.cppm
)
target_link_libraries(harmonius-rg-diag
    PUBLIC harmonius-rg-core harmonius-gpu
)

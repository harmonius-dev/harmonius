add_library(harmonius-rg-builder STATIC)
target_sources(harmonius-rg-builder PUBLIC FILE_SET CXX_MODULES FILES harmonius.rg.builder.cppm)
target_link_libraries(harmonius-rg-builder PUBLIC harmonius-rg-core harmonius-gpu)

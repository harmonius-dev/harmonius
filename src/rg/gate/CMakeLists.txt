add_library(harmonius-rg-gate STATIC)
target_sources(harmonius-rg-gate PUBLIC FILE_SET CXX_MODULES FILES
                                        harmonius.rg.gate.cppm)
target_link_libraries(harmonius-rg-gate PUBLIC harmonius-rg-core)

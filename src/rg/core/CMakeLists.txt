add_library(harmonius-rg-core STATIC)
target_sources(harmonius-rg-core PUBLIC FILE_SET CXX_MODULES FILES
                                        harmonius.rg.cppm)
target_link_libraries(harmonius-rg-core PUBLIC harmonius-gpu)

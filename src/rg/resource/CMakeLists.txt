add_library(harmonius-rg-resource STATIC)
target_sources(harmonius-rg-resource PUBLIC FILE_SET CXX_MODULES FILES
                                            harmonius.rg.resource.cppm)
target_link_libraries(harmonius-rg-resource PUBLIC harmonius-rg-core
                                                   harmonius-gpu)

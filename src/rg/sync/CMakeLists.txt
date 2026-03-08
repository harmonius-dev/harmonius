add_library(harmonius-rg-sync STATIC)
target_sources(harmonius-rg-sync PUBLIC FILE_SET CXX_MODULES FILES
                                        harmonius.rg.sync.cppm)
target_link_libraries(
  harmonius-rg-sync PUBLIC harmonius-rg-core harmonius-rg-compiler
                           harmonius-gpu)

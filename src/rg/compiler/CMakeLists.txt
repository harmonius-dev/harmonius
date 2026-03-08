add_library(harmonius-rg-compiler STATIC)
target_sources(harmonius-rg-compiler PUBLIC FILE_SET CXX_MODULES FILES
                                            harmonius.rg.compiler.cppm)
target_link_libraries(
  harmonius-rg-compiler PUBLIC harmonius-rg-core harmonius-rg-builder
                               harmonius-gpu)

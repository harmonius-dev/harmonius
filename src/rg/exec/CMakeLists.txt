add_library(harmonius-rg-exec STATIC)
target_sources(harmonius-rg-exec PUBLIC FILE_SET CXX_MODULES FILES harmonius.rg.exec.cppm)
target_link_libraries(harmonius-rg-exec PUBLIC harmonius-rg-core harmonius-rg-compiler harmonius-gpu)

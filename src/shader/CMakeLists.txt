add_library(harmonius-shader STATIC)
target_sources(harmonius-shader PUBLIC FILE_SET CXX_MODULES FILES
                                       harmonius.shader.cppm)
target_link_libraries(harmonius-shader PUBLIC harmonius-gpu)

cmake_minimum_required(VERSION 3.12)
project(2023)

# Get a list of all .cpp files in the directory and its subdirectories
file(GLOB_RECURSE SOURCES "*.cpp")

# Loop over each source file and create an executable
foreach (SOURCE_FILE ${SOURCES})
    # Extract the file name without extension (e.g., A.cpp -> A)
    get_filename_component(EXECUTABLE_NAME ${SOURCE_FILE} NAME_WE)

    # Extract the relative path to the source file
    file(RELATIVE_PATH RELATIVE_PATH ${CMAKE_CURRENT_SOURCE_DIR} ${SOURCE_FILE})

    # Replace invalid characters in the relative path with underscores
    string(REGEX REPLACE "[^a-zA-Z0-9]" "_" RELATIVE_PATH ${RELATIVE_PATH})

    # Create an executable for each source file using a fixed prefix and unique name
    add_executable(${RELATIVE_PATH}_${EXECUTABLE_NAME} ${SOURCE_FILE})
endforeach ()

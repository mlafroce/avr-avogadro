cmake_minimum_required(VERSION 2.8)
project(qt5-demo)

# Find the QtWidgets library
find_package(Qt5Widgets)

message("[INFO] La raiz del proyecto es ${PROJECT_SOURCE_DIR}")

# Agrego ej3/include a la lista de directorios de includes
include_directories(${PROJECT_SOURCE_DIR}/include)

# Esta macro toma cada uno los parámetros y ejecuta *uic* sobre ellos
# La aplicacion uic genera headers con la definición de los widgets
# a partir de un archivo .ui
# Los nombres de los headers generados se guardan en la variable pasada
# como primer parámetro.
# No, no los genera con la carpeta como prefijo.
qt5_wrap_ui(UIS_HEADERS
        ui/MainWindow.ui
)

# Agrego la carpeta con los headers autogenerados a los includes
message("Agregando '${PROJECT_BINARY_DIR}' como directorio de includes")
include_directories(${PROJECT_BINARY_DIR})

message("Headers generados: ${UIS_HEADERS}")

add_library(avogadrogui
  api.cpp
  src/MainWindow.cpp
  ${UIS_HEADERS})

# add_executable(avogadro-test
#  main.cpp)

# Use the Widgets module from Qt 5.
target_link_libraries(avogadrogui Qt5::Widgets)
# target_link_libraries(avogadro-test avogadrogui)

install(TARGETS avogadrogui DESTINATION .)


# List all the header files
set(HEADERS
        Setting.h
        Gun.h
        Player.h
        Team.h
        Guns.h
        Time.h
        Handler.h
        Logger.h
        Error.h
)

# List all the source files
set(SOURCES
        Setting.cpp
        Player.cpp
        Team.cpp
        Time.cpp
        Handler.cpp
        Logger.cpp
        main.cpp
)

# List all the inline header files
set(INLINE_HEADERS
        Guns-inl.h
        Gun-inl.h
)

# List all the temporary header files
set(TEMP_HEADERS
)

# Add the executable
add_executable(Counter_Strike_Quera ${SOURCES} ${HEADERS} ${INLINE_HEADERS} ${TEMP_HEADERS})

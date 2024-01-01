#!/bin/bash

intro() {
    echo "-----------------------------"
    echo "Bullet Toolchain Installer  (v1.0)"
    echo ""
    echo "1] Build components"
    echo "2] Install components"
    echo "3] Build and install components (RECOMMENDED)"
    shell_1()
}
shell_1() {
    read -p "[1/2/3]>" INPUT
    case $INPUT in
        1)
            build_component_menu()
            ;;
        2)
            install_component_menu()
            ;;
        3)
            build_install_component_menu()
            ;;
        *)
            echo "[!] Invalid input"
            echo ""
            shell_1()
    esac
}

shell_2() {
    read -p "[1/2/3]>" INPUT
    case $INPUT in
        1)
            build("bpkg")
            ;;
        2)
            build("abs")
            ;;
        3)
            build("bpkg")
            build("abs")
            ;;
        *)
            echo "[!] Invalid input"
            echo ""
            shell_1()
    esac
}
shell_3() {
    read -p "[1/2/3]>" INPUT
    case $INPUT in
        1)
            install("bpkg")
            ;;
        2)
            install("abs")
            ;;
        3)
            install("bpkg")
            install("abs")
            ;;
        *)
            echo "[!] Invalid input"
            echo ""
            shell_1()
    esac
}
shell_4() {
    read -p "[1/2/3]>" INPUT
    case $INPUT in
        1)
            build("bpkg")
            install("bpkg")
            ;;
        2)
            build("abs")
            install("abs")
            ;;
        3)
            build("bpkg")
            build("abs")
            install("bpkg")
            install("abs")
            ;;
        *)
            echo "[!] Invalid input"
            echo ""
            shell_1()
    esac
}

build_component_menu() {
    select_component()
    shell_2()
}
install_component_menu() {
    select_component()
    shell_3()
}
build_install_component_menu() {
    select_component()
    shell_4()
}
select_component() {
    clear
    echo "-----------------------------"
    echo "Select component:"
    echo "1] bpkg"
    echo "2] ABS"
    echo "3] bpkg+ABS (RECOMMENDED)"
}
compile() {
    # Compiles either bpkg or ABS
    cd $1
    cargo build --release
    cd ..
}
system_install() {
    # System installation for either bpkg or ABS
    cp -r ($1)/target/release/($1) /usr/bin
}
user_install() {
    # User installation for either bpkg or ABS
    mkdir ~/.bullet_toolchain
    cp -r ($1)/target/release/($1) ~/.bullet_toolchain
    echo "[!] Remember to add ~/.bullet_toolchain to $PATH"
}

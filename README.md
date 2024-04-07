# Fastsmb

Fastsmb was created to provide a means to quickly write files from the python 
language to SMB network shares such as netapps ontap.

It was created since it was observed that pysmbc was slow when writing to
servers in different regions.

To oversome speed up the file transfer fastsmb utilises the following:

- Fastsmb is written in rust but provides a python binding. It uses the library paveo
    
- It will read the source file into a buffer of 1MB and transfer this on each write

The following should be installed libsmbclient

Install the C dependencies on your system 🖥️

MacOS 🍎

Install samba with brew:

brew install samba
Debian based systems 🐧

Install libsmbclient with apt:

apt install -y libsmbclient


RedHat based systems 🐧

Install libsmbclient with dnf:

dnf install libsmbclient

Next you can pip install fastsmb

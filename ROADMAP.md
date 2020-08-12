# psswd: shell password manager roadmap

~~- add and encrypt password (using age's rcrypt)~~
~~- display plain password by decrypting psswd file through passphrase~~
~~- store psswd encrypted files in Home folder (depending on the user's OS) using dirs~~

- improve error handling 
  - during `psswd show` if passphrase is wrong
  - during `psswd add` if `.psswd` folder has not been created
  - during `psswd list`
- add `psswd delete` to be able to delete psswd file by specifying shortname
- add `psswd delete --all` to be able to delete entire .psswd folder
- handle import of CSV files through `psswd import <filename>`

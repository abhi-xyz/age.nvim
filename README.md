<div align="center">

# just.nvim

</div>

<div align="center">

**Neovim plugin for encrypting and decrypting text files inside neovim using [age](https://github.com/FiloSottile/age) with ease.**

</div>

## Table of Contents  

- [Installation](#installation)  
- [Usage](#usage)  
- [What is age?](#what-is-age)  

## Installation

Install Just using your favorite plugin manager. For example, with [lazy.nvim](https://github.com/folke/lazy.nvim):

```lua
-- ~/.config/nvim/lua/plugins/just.lua

{
    'abhi-xyz/just.nvim',
    cmd = { "Just", "J" },
    config = function()
      local key = require('key')

      require('just').setup({
        public_key = "ageXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
        private_key = key.private_key, -- defined in another lua file which is not included in git for safety
      })
    end
}
```
```lua
-- ~/.config/nvim/lua/key.lua

return {
  private_key = "AGE-SECRET-KEY-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
}
```

## Usage

Just provides the `:Just` command with the following syntax:

```vim
:Just [action]
```

- `[action]` can be one of:
  - `encrypt`,
  - `decrypt`,
  - `genkey`

#### Examples:

- Generates an age key pair into key.txt in current working directory.

```vim
:Just genkey
```

- Kills the current buffer and switches to a previous buffer or creates a scratch buffer in case there is no buffer to switch, then encrypts the file with the provided age key.

```vim
:Just encrypt
```

- Decrypts the currently opened encrypted file, and switches to the decrypted file. 
```vim
:Just decrypt
```
## What is age?

[age](https://age-encryption.org/) is a simple, modern and secure file encryption tool.

It features small explicit keys, no config options, and UNIX-style composability.

### Why Choose Age Over GPG?

1. **Simplicity**: Age has a straightforward syntax and intuitive design, making it easier to use without extensive documentation.
2. **Modern Cryptography**: Age uses state-of-the-art cryptographic algorithms like X25519, ChaCha20-Poly1305, and HMAC-SHA256, ensuring robust security.
3. **Minimal Attack Surface**: Age's codebase is minimal and easier to audit compared to the complex and extensive GPG ecosystem.
4. **Portable Keys**: Age uses compact, user-friendly key formats, which are easy to manage and transfer.
5. **Focused Use Case**: Age is purpose-built for encrypting files securely and efficiently, without the additional complexity of key management and email encryption that GPG supports.

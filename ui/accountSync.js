console.log("📦 AccountSync.js starting to load...")

class AccountSyncClass {
  constructor() {
    this.currentNetwork = null
    this.xrpl = null
    this._getAccounts = null
    this._setAccounts = null
    this._onNetworkChange = null
  }

  /**
   * Initialize the AccountSync with the XRPL library and current network
   * @param {object} xrplLib
   * @param {string} network
   */
  init(xrplLib, network) {
    this.xrpl = xrplLib
    this.currentNetwork = network
  }

  /**
   * Set the current network and optionally trigger account reload
   * @param {string} network
   * @param {boolean} autoLoad
   * @returns {Array}
   */
  setNetwork(network, autoLoad = true) {
    this.currentNetwork = network

    if (autoLoad && this._setAccounts) {
      const loaded = this.loadAccounts()
      this._setAccounts(loaded)

      if (this._onNetworkChange) {
        this._onNetworkChange(network, loaded)
      }

      return loaded
    }

    return []
  }

  /**
   * Enable auto-sync - automatically save/load accounts when they change
   * @param {Function} getAccounts
   * @param {Function} setAccounts
   * @param {Function} onNetworkChange
   */
  autoSync(getAccounts, setAccounts, onNetworkChange = null) {
    this._getAccounts = getAccounts
    this._setAccounts = setAccounts
    this._onNetworkChange = onNetworkChange

    console.log("✓ AccountSync auto-sync enabled")
  }

  /**
   * Manually trigger a save of current accounts
   * Call this after adding/removing/modifying accounts
   * @returns {boolean}
   */
  sync() {
    if (this._getAccounts) {
      const accounts = this._getAccounts()
      return this.saveAccounts(accounts)
    }
    console.warn(
      "AccountSync: sync() called but getAccounts not set. Call autoSync() first.",
    )
    return false
  }

  /**
   * Get the storage key for the current network
   * @returns {string}
   */
  getStorageKey() {
    if (!this.currentNetwork) {
      console.warn("No network set, using default key")
      return "xrpl_accounts_default"
    }
    const sanitized = this.currentNetwork.replace(/[^a-zA-Z0-9]/g, "_")
    return `xrpl_accounts_${sanitized}`
  }

  /**
   * Save accounts to localStorage for the current network
   * @param {Array} accounts
   * @returns {boolean}
   */
  saveAccounts(accounts) {
    try {
      const simpleAccounts = accounts.map((acc) => ({
        address: acc.address,
        publicKey: acc.publicKey,
        privateKey: acc.privateKey,
        seed: acc.seed,
      }))

      const key = this.getStorageKey()
      localStorage.setItem(key, JSON.stringify(simpleAccounts))

      console.log(`✓ Saved ${accounts.length} account(s) to ${key}`)
      return true
    } catch (error) {
      console.error("Failed to save accounts:", error)
      return false
    }
  }

  /**
   * Load accounts from localStorage for the current network
   * @returns {Array}
   */
  loadAccounts() {
    try {
      const key = this.getStorageKey()
      const data = localStorage.getItem(key)

      if (!data) {
        console.log(`No accounts found for ${key}`)
        return []
      }

      if (!this.xrpl) {
        console.error("XRPL library not initialized")
        return []
      }

      const arr = JSON.parse(data)
      const loadedAccounts = arr.map((acc) =>
        this.xrpl.Wallet.fromSeed(acc.seed),
      )

      console.log(`✓ Loaded ${loadedAccounts.length} account(s) from ${key}`)
      return loadedAccounts
    } catch (error) {
      console.error("Failed to load accounts:", error)
      return []
    }
  }

  /**
   * Clear accounts for the current network
   * @returns {boolean}
   */
  clearAccounts() {
    try {
      const key = this.getStorageKey()
      localStorage.removeItem(key)
      console.log(`✓ Cleared accounts for ${key}`)
      return true
    } catch (error) {
      console.error("Failed to clear accounts:", error)
      return false
    }
  }

  /**
   * Get all networks that have stored accounts
   * @returns {Array}
   */
  getAllNetworks() {
    const networks = []
    const prefix = "xrpl_accounts_"

    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i)
      if (key && key.startsWith(prefix)) {
        networks.push(key.substring(prefix.length))
      }
    }

    return networks
  }

  /**
   * Get the count of accounts for a specific network
   * @param {string} network
   * @returns {number}
   */
  getAccountCount(network = null) {
    try {
      const savedNetwork = this.currentNetwork
      if (network) {
        this.currentNetwork = network
      }

      const key = this.getStorageKey()
      const data = localStorage.getItem(key)

      if (network) {
        this.currentNetwork = savedNetwork
      }

      if (!data) return 0

      const arr = JSON.parse(data)
      return arr.length
    } catch (error) {
      console.error("Failed to get account count:", error)
      return 0
    }
  }

  /**
   * Export accounts for the current network as JSON
   * @returns {string}
   */
  exportAccounts() {
    try {
      const key = this.getStorageKey()
      const data = localStorage.getItem(key)

      if (!data) return null

      const accounts = JSON.parse(data)
      return JSON.stringify(accounts, null, 2)
    } catch (error) {
      console.error("Failed to export accounts:", error)
      return null
    }
  }

  /**
   * Import accounts for the current network from JSON
   * @param {string} jsonData
   * @param {boolean} append
   * @returns {number}
   */
  importAccounts(jsonData, append = false) {
    try {
      const newAccounts = JSON.parse(jsonData)

      if (!Array.isArray(newAccounts)) {
        throw new Error("Invalid format: expected array of accounts")
      }

      for (const acc of newAccounts) {
        if (!acc.address || !acc.seed) {
          throw new Error("Invalid account structure: missing address or seed")
        }
      }

      let accountsToSave = newAccounts

      if (append) {
        const key = this.getStorageKey()
        const existingData = localStorage.getItem(key)
        if (existingData) {
          const existingAccounts = JSON.parse(existingData)
          accountsToSave = [...existingAccounts, ...newAccounts]
        }
      }

      const key = this.getStorageKey()
      localStorage.setItem(key, JSON.stringify(accountsToSave))

      console.log(`✓ Imported ${newAccounts.length} account(s)`)
      return newAccounts.length
    } catch (error) {
      console.error("Failed to import accounts:", error)
      throw error
    }
  }

  /**
   * Clear all accounts across all networks
   * @returns {number}
   */
  clearAllNetworks() {
    try {
      const networks = this.getAllNetworks()
      const prefix = "xrpl_accounts_"

      networks.forEach((network) => {
        localStorage.removeItem(`${prefix}${network}`)
      })

      console.log(`✓ Cleared accounts for ${networks.length} network(s)`)
      return networks.length
    } catch (error) {
      console.error("Failed to clear all networks:", error)
      return 0
    }
  }

  /**
   * Remove a single account by index
   * @param {number} index
   * @returns {boolean}
   */
  removeAccount(index) {
    if (!this._getAccounts || !this._setAccounts) {
      console.warn(
        "AccountSync: removeAccount() called but autoSync not enabled",
      )
      return false
    }

    try {
      const accounts = this._getAccounts()
      if (index < 0 || index >= accounts.length) {
        console.error(`Invalid account index: ${index}`)
        return false
      }

      accounts.splice(index, 1)
      this._setAccounts(accounts)
      this.sync()

      console.log(`✓ Removed account at index ${index}`)
      return true
    } catch (error) {
      console.error("Failed to remove account:", error)
      return false
    }
  }

  /**
   * Add a new account
   * @param {object} account
   * @returns {boolean}
   */
  addAccount(account) {
    if (!this._getAccounts || !this._setAccounts) {
      console.warn("AccountSync: addAccount() called but autoSync not enabled")
      return false
    }

    try {
      const accounts = this._getAccounts()
      accounts.push(account)
      this._setAccounts(accounts)
      this.sync()

      console.log(`✓ Added account ${account.address}`)
      return true
    } catch (error) {
      console.error("Failed to add account:", error)
      return false
    }
  }

  /**
   * Load accounts for the current network on demand
   * Useful for initial page load
   * @returns {Object}
   */
  initialize() {
    if (!this._setAccounts) {
      console.warn("AccountSync: initialize() called but autoSync not enabled")
      return { accounts: [], count: 0, message: null }
    }

    const loaded = this.loadAccounts()
    this._setAccounts(loaded)

    const count = loaded.length
    let message = null

    if (count > 0) {
      message = `Loaded ${count} account${count > 1 ? "s" : ""} from storage`
      console.log(`✓ Initialized with ${count} account(s) for current network`)
    }

    return { accounts: loaded, count, message }
  }
}

;(function () {
  const accountSync = new AccountSyncClass()

  if (typeof window !== "undefined") {
    window.AccountSync = accountSync
    console.log("✓ AccountSync singleton loaded and available globally")
    console.log("✓ AccountSync.init is:", typeof accountSync.init)
  }

  if (typeof module !== "undefined" && module.exports) {
    module.exports = accountSync
  }
})()

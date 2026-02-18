console.log("📦 EscrowSync.js starting to load...")

class EscrowSyncClass {
  constructor() {
    this.currentNetwork = null
    this._getEscrows = null
    this._setEscrows = null
  }

  /**
   * Initialize the EscrowSync with the current network
   * @param {string} network
   */
  init(network) {
    this.currentNetwork = network
  }

  /**
   * Set the current network and optionally trigger escrow reload
   * @param {string} network
   * @param {boolean} autoLoad
   * @returns {Array}
   */
  setNetwork(network, autoLoad = true) {
    this.currentNetwork = network

    if (autoLoad && this._setEscrows) {
      const loaded = this.loadEscrows()
      this._setEscrows(loaded)
      return loaded
    }

    return []
  }

  /**
   * Enable auto-sync - automatically save/load escrows when they change
   * @param {Function} getEscrows
   * @param {Function} setEscrows
   */
  autoSync(getEscrows, setEscrows) {
    this._getEscrows = getEscrows
    this._setEscrows = setEscrows

    console.log("✓ EscrowSync auto-sync enabled")
  }

  /**
   * Manually trigger a save of current escrows
   * Call this after adding/removing/modifying escrows
   * @returns {boolean}
   */
  sync() {
    if (this._getEscrows) {
      const escrows = this._getEscrows()
      return this.saveEscrows(escrows)
    }
    console.warn(
      "EscrowSync: sync() called but getEscrows not set. Call autoSync() first.",
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
      return "xrpl_escrows_default"
    }
    const sanitized = this.currentNetwork.replace(/[^a-zA-Z0-9]/g, "_")
    return `xrpl_escrows_${sanitized}`
  }

  /**
   * Save escrows to localStorage for the current network
   * @param {Array} escrows
   * @returns {boolean}
   */
  saveEscrows(escrows) {
    try {
      const key = this.getStorageKey()
      localStorage.setItem(key, JSON.stringify(escrows))

      console.log(`✓ Saved ${escrows.length} escrow(s) to ${key}`)
      return true
    } catch (error) {
      console.error("Failed to save escrows:", error)
      return false
    }
  }

  /**
   * Load escrows from localStorage for the current network
   * @returns {Array}
   */
  loadEscrows() {
    try {
      const key = this.getStorageKey()
      const data = localStorage.getItem(key)

      if (!data) {
        console.log(`No escrows found for ${key}`)
        return []
      }

      const escrows = JSON.parse(data)

      console.log(`✓ Loaded ${escrows.length} escrow(s) from ${key}`)
      return escrows
    } catch (error) {
      console.error("Failed to load escrows:", error)
      return []
    }
  }

  /**
   * Clear escrows for the current network
   * @returns {boolean}
   */
  clearEscrows() {
    try {
      const key = this.getStorageKey()
      localStorage.removeItem(key)
      console.log(`✓ Cleared escrows for ${key}`)
      return true
    } catch (error) {
      console.error("Failed to clear escrows:", error)
      return false
    }
  }

  /**
   * Remove a single escrow by index
   * @param {number} index
   * @returns {boolean}
   */
  removeEscrow(index) {
    if (!this._getEscrows || !this._setEscrows) {
      console.warn("EscrowSync: removeEscrow() called but autoSync not enabled")
      return false
    }

    try {
      const escrows = this._getEscrows()
      if (index < 0 || index >= escrows.length) {
        console.error(`Invalid escrow index: ${index}`)
        return false
      }

      escrows.splice(index, 1)
      this._setEscrows(escrows)
      this.sync()

      console.log(`✓ Removed escrow at index ${index}`)
      return true
    } catch (error) {
      console.error("Failed to remove escrow:", error)
      return false
    }
  }

  /**
   * Add a new escrow (called after WASM creates one)
   * @param {object} escrow
   * @returns {boolean}
   */
  addEscrow(escrow) {
    if (!this._getEscrows || !this._setEscrows) {
      console.warn("EscrowSync: addEscrow() called but autoSync not enabled")
      return false
    }

    try {
      const escrows = this._getEscrows()
      escrows.push(escrow)
      this._setEscrows(escrows)
      this.sync()

      console.log(`✓ Added escrow (sequence: ${escrow.sequence})`)
      return true
    } catch (error) {
      console.error("Failed to add escrow:", error)
      return false
    }
  }

  /**
   * Export escrows for the current network as JSON
   * @returns {string}
   */
  exportEscrows() {
    try {
      const key = this.getStorageKey()
      const data = localStorage.getItem(key)

      if (!data) return null

      const escrows = JSON.parse(data)
      return JSON.stringify(escrows, null, 2)
    } catch (error) {
      console.error("Failed to export escrows:", error)
      return null
    }
  }

  /**
   * Load escrows for the current network on demand
   * Useful for initial page load
   * @returns {Object}
   */
  initialize() {
    if (!this._setEscrows) {
      console.warn("EscrowSync: initialize() called but autoSync not enabled")
      return { escrows: [], count: 0, message: null }
    }

    const loaded = this.loadEscrows()
    this._setEscrows(loaded)

    const count = loaded.length
    let message = null

    if (count > 0) {
      message = `Loaded ${count} escrow${count > 1 ? "s" : ""} from storage`
      console.log(`✓ Initialized with ${count} escrow(s) for current network`)
    }

    return { escrows: loaded, count, message }
  }

  /**
   * Get the count of escrows for the current network
   * @returns {number}
   */
  getEscrowCount() {
    try {
      const key = this.getStorageKey()
      const data = localStorage.getItem(key)

      if (!data) return 0

      const escrows = JSON.parse(data)
      return escrows.length
    } catch (error) {
      console.error("Failed to get escrow count:", error)
      return 0
    }
  }
}

;(function () {
  const escrowSync = new EscrowSyncClass()

  if (typeof window !== "undefined") {
    window.EscrowSync = escrowSync
    console.log("✓ EscrowSync singleton loaded and available globally")
  }
  if (typeof module !== "undefined" && module.exports) {
    module.exports = escrowSync
  }
})()

/**
 * FILE:xletDispatcher.js
 * @short_description: Modular dispatcher for xlet (Applet, Extension, etc) loading.
 */
const AppletManager = imports.ui.appletManager;
const ExtensionSystem = imports.ui.extensionSystem;
const DeskletManager = imports.ui.deskletManager;
const SearchProviderManager = imports.ui.searchProviderManager;

var XletDispatcher = class {
    constructor() {
        this._managers = [
            AppletManager,
            ExtensionSystem,
            DeskletManager,
            SearchProviderManager
        ];
    }

    async initAll() {
        global.log("Sylva: Initializing Xlet systems via Dispatcher...");
        
        let initPromises = this._managers.map(manager => {
            try {
                return manager.init();
            } catch (e) {
                global.logError(`Dispatcher: Failed to initialize ${manager}: ${e}`);
                return Promise.resolve();
            }
        });

        return Promise.all(initPromises);
    }
};

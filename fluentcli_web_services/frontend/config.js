// This file overrides the API URL at runtime
window.API_URL = 'http://localhost';
console.log('Runtime config loaded: API_URL =', window.API_URL);

// Intercept fetch and XMLHttpRequest to log and fix URLs
(function() {
    // Store original fetch
    const originalFetch = window.fetch;

    // Override fetch
    window.fetch = function(url, options) {
        // Check if the URL contains port 8001
        if (typeof url === 'string' && url.includes('8001')) {
            console.warn('Detected port 8001 in fetch URL:', url);
            // Replace port 8001 with default port (80)
            url = url.replace(':8001', '');
            console.log('Fixed URL:', url);
        }

        // Call original fetch with possibly modified URL
        return originalFetch.apply(this, [url, options]);
    };

    // Store original XMLHttpRequest open method
    const originalOpen = XMLHttpRequest.prototype.open;

    // Override XMLHttpRequest open method
    XMLHttpRequest.prototype.open = function(method, url, async, user, password) {
        let modifiedUrl = url;
        
        // Check if the URL contains port 8001
        if (typeof url === 'string' && url.includes('8001')) {
            console.warn('Detected port 8001 in XMLHttpRequest URL:', url);
            // Replace port 8001 with default port (80)
            modifiedUrl = url.replace(':8001', '');
            console.log('Fixed URL:', modifiedUrl);
        }
        
        // Fix double slashes in URL paths (except after protocol)
        if (typeof modifiedUrl === 'string' && modifiedUrl.includes('//')) {
            const fixedUrl = modifiedUrl.replace(/(https?:\/\/)|(\/\/)/g, function(match, protocol) {
                return protocol || '/';
            });
            
            if (fixedUrl !== modifiedUrl) {
                console.warn('Fixed double slashes in URL:', modifiedUrl, '->', fixedUrl);
                modifiedUrl = fixedUrl;
            }
        }

        // Call original open with modified URL
        return originalOpen.call(this, method, modifiedUrl, async, user, password);
    };

    console.log('Network request interceptors installed');
})();

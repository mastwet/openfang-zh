/**
 * OpenFang i18n (Internationalization) Module
 * 
 * Provides runtime language switching for the OpenFang dashboard UI.
 * Supports Chinese (default), English, and Russian.
 * 
 * Usage:
 *   - HTML: <span data-i18n="nav.overview">Overview</span>
 *   - JS:   window.i18n.t('nav.overview')
 *   - Alpine: x-text="window.i18n.t('nav.overview')"
 *   - Auto-applies translations on load based on stored/preferred language
 *   - Re-applies after Alpine.js initialization
 */

(function() {
  'use strict';

  // Language store — default to Chinese (this is a zh-localized build)
  let currentLang = 'zh';
  let translations = {};
  let isInitialized = false;

  /**
   * Load translations from a JSON file
   * @param {string} lang - Language code (en, ru, zh)
   * @returns {Promise<Object>} Translation object
   */
  async function loadTranslations(lang) {
    try {
      // Use cached translations if available
      if (window.__i18nCache && window.__i18nCache[lang]) {
        return window.__i18nCache[lang];
      }

      const response = await fetch('/i18n/' + lang + '.json');
      if (!response.ok) {
        console.warn('[i18n] Failed to load ' + lang + '.json (status ' + response.status + '), falling back to en');
        if (lang !== 'en') {
          return loadTranslations('en');
        }
        return {};
      }

      var data = await response.json();
      
      // Cache for future use
      if (!window.__i18nCache) window.__i18nCache = {};
      window.__i18nCache[lang] = data;
      
      console.log('[i18n] Loaded ' + Object.keys(data).length + ' translations for ' + lang);
      return data;
    } catch (error) {
      console.error('[i18n] Error loading translations for ' + lang + ':', error);
      if (lang !== 'en') {
        return loadTranslations('en');
      }
      return {};
    }
  }

  /**
   * Get a translated string by key
   * @param {string} key - Translation key (e.g., 'nav.overview')
   * @param {Object} params - Optional interpolation parameters
   * @returns {string} Translated string or key if not found
   */
  function t(key, params) {
    // If not initialized yet, try using cached translations from a previous session
    var text = translations[key];
    if (!text) {
      // Return the key as fallback — this is better than showing nothing
      text = key;
    }

    // Handle interpolation (e.g., 'Hello, {{name}}')
    if (params && typeof params === 'object') {
      Object.keys(params).forEach(function(param) {
        text = text.replace(new RegExp('{{' + param + '}}', 'g'), params[param]);
      });
    }

    return text;
  }

  /**
   * Apply translations to all elements with data-i18n attribute
   * Also updates the <html> lang attribute
   */
  function applyTranslations() {
    // Update document language
    document.documentElement.lang = currentLang;
    
    // Find and translate all elements with data-i18n attribute
    var elements = document.querySelectorAll('[data-i18n]');
    elements.forEach(function(el) {
      var key = el.getAttribute('data-i18n');
      var translation = t(key);
      
      // Skip if translation equals key (not found) — keep original text
      if (translation === key) return;

      // Check if element is a form input/textarea
      if (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA') {
        // For form elements, only update if it has a placeholder or aria-label
        if (el.hasAttribute('data-i18n-placeholder') || el.hasAttribute('placeholder')) {
          el.placeholder = translation;
        }
        if (el.hasAttribute('aria-label')) {
          el.setAttribute('aria-label', translation);
        }
        if (el.hasAttribute('title')) {
          el.setAttribute('title', translation);
        }
      } else {
        // For regular elements, update text content
        el.textContent = translation;
      }
    });

    // Update elements with data-i18n-placeholder attribute
    var placeholderElements = document.querySelectorAll('[data-i18n-placeholder]');
    placeholderElements.forEach(function(el) {
      var key = el.getAttribute('data-i18n-placeholder');
      var translation = t(key);
      if (translation !== key) {
        el.placeholder = translation;
      }
    });

    // Update elements with data-i18n-title attribute
    var titleElements = document.querySelectorAll('[data-i18n-title]');
    titleElements.forEach(function(el) {
      var key = el.getAttribute('data-i18n-title');
      var translation = t(key);
      if (translation !== key) {
        el.title = translation;
      }
    });

    // Update elements with data-i18n-aria-label attribute
    var ariaElements = document.querySelectorAll('[data-i18n-aria-label]');
    ariaElements.forEach(function(el) {
      var key = el.getAttribute('data-i18n-aria-label');
      var translation = t(key);
      if (translation !== key) {
        el.setAttribute('aria-label', translation);
      }
    });

    console.log('[i18n] Applied translations for language: ' + currentLang);
  }

  /**
   * Set the current language and apply translations
   * @param {string} lang - Language code (en, ru, zh)
   * @param {boolean} persist - Whether to save to localStorage
   */
  async function setLanguage(lang, persist) {
    if (persist === undefined) persist = true;
    if (!['en', 'ru', 'zh'].includes(lang)) {
      console.warn('[i18n] Unknown language: ' + lang + ', defaulting to zh');
      lang = 'zh';
    }

    currentLang = lang;
    translations = await loadTranslations(lang);
    isInitialized = true;

    // Save preference
    if (persist) {
      localStorage.setItem('openfang_language', lang);
    }

    // Apply to DOM
    applyTranslations();

    // Dispatch event for Alpine.js components to react
    window.dispatchEvent(new CustomEvent('i18n:language-changed', { 
      detail: { language: lang } 
    }));
  }

  /**
   * Get the current language
   * @returns {string} Current language code
   */
  function getLanguage() {
    return currentLang;
  }

  /**
   * Initialize i18n system
   * Loads language preference and applies translations
   */
  async function init() {
    // Determine language priority:
    // 1. localStorage (user preference)
    // 2. Browser language
    // 3. Default to Chinese (this is a zh-localized build)

    var lang = localStorage.getItem('openfang_language');
    
    if (!lang) {
      // Try to detect browser language
      var browserLang = navigator.language || navigator.userLanguage || '';
      if (browserLang.startsWith('zh')) {
        lang = 'zh';
      } else if (browserLang.startsWith('ru')) {
        lang = 'ru';
      } else {
        lang = 'zh'; // Default to Chinese for this build
      }
    }

    await setLanguage(lang, false);

    // Re-apply translations after Alpine.js has initialized
    // Alpine renders x-if/x-for templates after init, so we need
    // to re-apply translations to catch newly rendered elements.
    if (typeof Alpine !== 'undefined') {
      // Alpine is already loaded — re-apply after a tick
      setTimeout(applyTranslations, 100);
      setTimeout(applyTranslations, 500);
      setTimeout(applyTranslations, 1500);
    } else {
      // Alpine not yet loaded — listen for its init event
      document.addEventListener('alpine:init', function() {
        // Alpine is about to initialize — schedule re-apply
        setTimeout(applyTranslations, 200);
      });
      document.addEventListener('alpine:initialized', function() {
        // Alpine has finished initializing — final re-apply
        setTimeout(applyTranslations, 100);
        setTimeout(applyTranslations, 500);
      });
    }

    // Also re-apply on any page navigation (hash change)
    window.addEventListener('hashchange', function() {
      setTimeout(applyTranslations, 150);
      setTimeout(applyTranslations, 500);
    });
  }

  /**
   * Get available languages
   * @returns {Array<{code: string, name: string}>}
   */
  function getAvailableLanguages() {
    return [
      { code: 'zh', name: '简体中文' },
      { code: 'en', name: 'English' },
      { code: 'ru', name: 'Русский' }
    ];
  }

  // Expose to global scope
  window.i18n = {
    t: t,
    setLanguage: setLanguage,
    getLanguage: getLanguage,
    getAvailableLanguages: getAvailableLanguages,
    init: init,
    isInitialized: function() { return isInitialized; },
    applyTranslations: applyTranslations
  };

  // Also expose as window.t for convenient Alpine usage
  window.t = t;

  // Auto-initialize when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }

})();
(function () {
    if (window.__resizableInit) return;
    window.__resizableInit = true;

    const SIZES = {
        Desktop: null,   // full width
        Tablet: 768,
        Phone: 375,
    };

    function setSizeByScreenType(instanceId, screenType) {
        let containers;
        if (instanceId) {
            const root = document.querySelector(`[data-resizable="${instanceId}"]`);
            if (!root) return;
            containers = root.querySelectorAll('[data-resizable-container]');
        } else {
            containers = document.querySelectorAll('[data-resizable-container]');
        }

        const maxWidth = SIZES[screenType] ?? null;
        containers.forEach(c => {
            if (maxWidth) {
                c.style.maxWidth = maxWidth + 'px';
            } else {
                c.style.maxWidth = '';
            }
        });
    }

    document.addEventListener('resizable:resize_by_screen__interop', function (e) {
        const { instanceId, screenType } = e.detail || {};
        setSizeByScreenType(instanceId, screenType);
    });

    // Drag handle support
    document.addEventListener('mousedown', function (e) {
        const handle = e.target.closest('[data-resizable-handle]');
        if (!handle) return;

        const resizable = handle.closest('[data-resizable]');
        if (!resizable) return;
        const container = resizable.querySelector('[data-resizable-container]');
        if (!container) return;

        e.preventDefault();
        const startX = e.clientX;
        const startWidth = container.getBoundingClientRect().width;

        function onMove(e) {
            const delta = e.clientX - startX;
            const newWidth = Math.max(150, startWidth + delta);
            container.style.maxWidth = newWidth + 'px';
        }

        function onUp() {
            document.removeEventListener('mousemove', onMove);
            document.removeEventListener('mouseup', onUp);
        }

        document.addEventListener('mousemove', onMove);
        document.addEventListener('mouseup', onUp);
    });
})();

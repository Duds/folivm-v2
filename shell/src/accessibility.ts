/**
 * Maintain an ARIA shadow tree parallel to the canvas for screen reader support.
 *
 * The canvas element is not intrinsically accessible. This module maintains a
 * hidden DOM tree that mirrors the document structure. Screen readers navigate
 * this shadow tree while the visual editor runs on the canvas.
 *
 * ARIA nodes are emitted by folivm-core's render pipeline and forwarded here
 * via the WASM→JS callback interface.
 */

export interface AriaNode {
  id: string
  role: string
  label?: string
  children?: AriaNode[]
}

let ariaContainer: HTMLElement | null = null

/**
 * Recursively build DOM elements from AriaNode tree.
 */
function buildARIAElement(node: AriaNode): HTMLElement {
  const el = document.createElement('div')
  el.id = node.id
  el.setAttribute('role', node.role)
  if (node.label) {
    el.setAttribute('aria-label', node.label)
  }

  if (node.children) {
    for (const child of node.children) {
      el.appendChild(buildARIAElement(child))
    }
  }

  return el
}

/**
 * Apply a diff of ARIA nodes to the shadow DOM tree.
 * Called whenever folivm-core emits an accessibility update.
 */
export function updateARIA(nodes: AriaNode[]): void {
  if (!ariaContainer) return

  // For now, simply rebuild the tree from scratch.
  // A more sophisticated implementation would compute a diff and apply minimal updates.
  ariaContainer.innerHTML = ''
  for (const node of nodes) {
    ariaContainer.appendChild(buildARIAElement(node))
  }
}

/**
 * Initialise the ARIA shadow tree container and attach it to the document.
 * Must be called once before any updateARIA() calls.
 */
export function initARIA(): void {
  ariaContainer = document.createElement('div')
  ariaContainer.id = 'aria-tree'
  ariaContainer.setAttribute('role', 'presentation')
  ariaContainer.style.display = 'none'
  ariaContainer.setAttribute('aria-hidden', 'false')
  document.body.appendChild(ariaContainer)
}

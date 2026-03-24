import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * CARD COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [data-name="Card"]                                                    │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  [data-name="CardHeader"]  ← px-6 padding                       │   │
 * │   │  ┌─────────────────────────────────────────────────────────┐    │   │
 * │   │  │  [data-name="CardTitle"]  ← <h3> font-semibold          │    │   │
 * │   │  │  "Card Title"                                           │    │   │
 * │   │  └─────────────────────────────────────────────────────────┘    │   │
 * │   ├─────────────────────────────────────────────────────────────────┤   │
 * │   │  [data-name="CardContent"]                                      │   │
 * │   │  ┌─────────────────────────────────────────────────────────┐    │   │
 * │   │  │  [data-name="CardDescription"]  ← text-muted-foreground │    │   │
 * │   │  │  "Hello, this is a more detailed description..."        │    │   │
 * │   │  └─────────────────────────────────────────────────────────┘    │   │
 * │   ├─────────────────────────────────────────────────────────────────┤   │
 * │   │  [data-name="CardFooter"]  ← <footer> flex items-center         │   │
 * │   │  ┌───────────────┐                    ┌───────────────┐         │   │
 * │   │  │    Cancel     │                    │    Confirm    │         │   │
 * │   │  │   (outline)   │                    │   (primary)   │         │   │
 * │   │  └───────────────┘                    └───────────────┘         │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * CARD STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │   .bg-card              ← Theme-aware background                        │
 * │   .text-card-foreground ← Theme-aware text                              │
 * │   .rounded-xl           ← Large rounded corners                         │
 * │   .border               ← Subtle border                                 │
 * │   .shadow-sm            ← Light drop shadow                             │
 * │   .flex.flex-col.gap-4  ← Vertical layout                               │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class CardPage extends BasePage {
  protected readonly componentName = "card";

  readonly card: Locator;
  readonly cardHeader: Locator;
  readonly cardTitle: Locator;
  readonly cardContent: Locator;
  readonly cardDescription: Locator;
  readonly cardFooter: Locator;
  readonly cancelButton: Locator;
  readonly confirmButton: Locator;

  constructor(page: Page) {
    super(page);

    this.card = this.preview.locator('[data-name="Card"]').first();
    this.cardHeader = this.preview.locator('[data-name="CardHeader"]').first();
    this.cardTitle = this.preview.locator('[data-name="CardTitle"]').first();
    this.cardContent = this.preview.locator('[data-name="CardContent"]').first();
    this.cardDescription = this.preview.locator('[data-name="CardDescription"]').first();
    this.cardFooter = this.preview.locator('[data-name="CardFooter"]').first();

    this.cancelButton = this.preview.getByRole("button", { name: "Cancel" });
    this.confirmButton = this.preview.getByRole("button", { name: "Confirm" });
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Card Page", () => {
  /**
   * STRUCTURE TESTS - Verify component hierarchy
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │  Card                                                           │
   * │  ├── CardHeader                                                 │
   * │  │   └── CardTitle (<h3>)                                       │
   * │  ├── CardContent                                                │
   * │  │   └── CardDescription                                        │
   * │  └── CardFooter (<footer>)                                      │
   * │      ├── Cancel button (outline)                                │
   * │      └── Confirm button (primary)                               │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Structure", () => {
    /**
     * TEST: Card Base Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │   [data-name="Card"]                                    │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  .bg-card           ← Theme background          │   │
     *   │   │  .text-card-foreground  ← Theme text            │   │
     *   │   │  .rounded-xl        ← Corner radius             │   │
     *   │   │  .border            ← Border styling            │   │
     *   │   │  .shadow-sm         ← Drop shadow               │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Card component has all required base styling classes
     */
    test("should render Card with correct base classes", async ({ page }) => {
      const ui = new CardPage(page);
      await ui.goto();

      await expect(ui.card).toHaveClass(/bg-card/);
      await expect(ui.card).toHaveClass(/text-card-foreground/);
      await expect(ui.card).toHaveClass(/rounded-xl/);
      await expect(ui.card).toHaveClass(/border/);
      await expect(ui.card).toHaveClass(/shadow-sm/);
    });

    /**
     * TEST: CardHeader with CardTitle
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [data-name="CardHeader"]       ← VISIBLE               │
     *   │  ┌─────────────────────────────────────────────────┐    │
     *   │  │  [data-name="CardTitle"] "Card Title" ← VISIBLE │    │
     *   │  └─────────────────────────────────────────────────┘    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CardHeader exists and contains CardTitle with correct text
     */
    test("should have CardHeader with CardTitle", async ({ page }) => {
      const ui = new CardPage(page);
      await ui.goto();

      await expect(ui.cardHeader).toBeVisible();
      await expect(ui.cardTitle).toBeVisible();
      await expect(ui.cardTitle).toHaveText("Card Title");
    });

    /**
     * TEST: CardContent with CardDescription
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [data-name="CardContent"]      ← VISIBLE               │
     *   │  ┌─────────────────────────────────────────────────┐    │
     *   │  │  [data-name="CardDescription"]   ← VISIBLE      │    │
     *   │  │  "Hello, this is a more detailed..."             │    │
     *   │  └─────────────────────────────────────────────────┘    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CardContent exists and contains CardDescription with text
     */
    test("should have CardContent with CardDescription", async ({ page }) => {
      const ui = new CardPage(page);
      await ui.goto();

      await expect(ui.cardContent).toBeVisible();
      await expect(ui.cardDescription).toBeVisible();
      await expect(ui.cardDescription).toContainText(
        "Hello, this is a more detailed description"
      );
    });

    /**
     * TEST: CardFooter with Buttons
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [data-name="CardFooter"]  (<footer>)   ← VISIBLE       │
     *   │  ┌──────────┐                ┌───────────┐              │
     *   │  │  Cancel  │                │  Confirm  │              │
     *   │  └──────────┘                └───────────┘              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CardFooter exists with Cancel and Confirm buttons
     */
    test("should have CardFooter with buttons", async ({ page }) => {
      const ui = new CardPage(page);
      await ui.goto();

      await expect(ui.cardFooter).toBeVisible();
      await expect(ui.cancelButton).toBeVisible();
      await expect(ui.confirmButton).toBeVisible();
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: CardTitle HTML Element Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │   <h3>Card Title</h3>                                   │
     *   │     ↑ tagName.toLowerCase() === "h3"                    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CardTitle renders as semantic h3 heading element
     */
    test("CardTitle should be an h3 element", async ({ page }) => {
      const ui = new CardPage(page);
      await ui.goto();

      const tagName = await ui.cardTitle.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("h3");
    });

    /**
     * TEST: CardFooter HTML Element Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │   <footer> ... </footer>                                │
     *   │     ↑ tagName.toLowerCase() === "footer"                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CardFooter renders as semantic footer element
     */
    test("CardFooter should be a footer element", async ({ page }) => {
      const ui = new CardPage(page);
      await ui.goto();

      const tagName = await ui.cardFooter.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("footer");
    });

    /**
     * TEST: CardTitle Font Weight
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  .font-semibold (600 weight)                            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CardTitle has semibold font weight for emphasis
     */
    test("CardTitle should have font-semibold class", async ({ page }) => {
      const ui = new CardPage(page);
      await ui.goto();

      await expect(ui.cardTitle).toHaveClass(/font-semibold/);
    });

    /**
     * TEST: CardDescription muted text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  .text-muted-foreground ← muted text color              │
     *   │  .text-sm               ← small font size               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CardDescription uses muted styling
     */
    test("CardDescription should have muted text classes", async ({ page }) => {
      const ui = new CardPage(page);
      await ui.goto();

      await expect(ui.cardDescription).toHaveClass(/text-muted-foreground/);
      await expect(ui.cardDescription).toHaveClass(/text-sm/);
    });

    /**
     * TEST: CardFooter flex layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  .flex .items-center .gap-2 ← button row layout         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CardFooter uses flex layout for button alignment
     */
    test("CardFooter should have flex layout classes", async ({ page }) => {
      const ui = new CardPage(page);
      await ui.goto();

      await expect(ui.cardFooter).toHaveClass(/flex/);
      await expect(ui.cardFooter).toHaveClass(/items-center/);
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Card is visible and accessible
     * ─────────────────────────────────────────────────────────────
     *
     *   Validates: Card and all sub-components are in the DOM and visible
     */
    test("all card sub-components should be visible", async ({ page }) => {
      const ui = new CardPage(page);
      await ui.goto();

      await expect(ui.card).toBeVisible();
      await expect(ui.cardHeader).toBeVisible();
      await expect(ui.cardTitle).toBeVisible();
      await expect(ui.cardContent).toBeVisible();
      await expect(ui.cardDescription).toBeVisible();
      await expect(ui.cardFooter).toBeVisible();
    });

    /**
     * TEST: Buttons are interactive
     * ─────────────────────────────────────────────────────────────
     *
     *   Validates: Cancel and Confirm buttons are enabled and clickable
     */
    test("footer buttons should be enabled", async ({ page }) => {
      const ui = new CardPage(page);
      await ui.goto();

      await expect(ui.cancelButton).toBeEnabled();
      await expect(ui.confirmButton).toBeEnabled();
    });
  });
});

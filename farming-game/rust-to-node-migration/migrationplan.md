# Farming Board Game: Rust TUI to Azure Web App Migration Plan

## Migration Goal

Convert the existing Rust-based Farming Game TUI application into a full-stack, real-time web application hosted primarily on Microsoft Azure services, following the component choices outlined in `techstacksummary.md`.

## Target Architecture (Based on `techstacksummary.md`)

*   **Frontend:** React + TypeScript
*   **Frontend Hosting:** Azure Static Web Apps (Includes CDN, SSL, CI/CD via GitHub Actions)
*   **Backend:** Node.js + TypeScript + Socket.IO (running in Docker)
*   **Backend Hosting:** Azure Container Apps (Preferred) or Azure App Service
*   **Database:** Azure PostgreSQL Flexible Server (Burstable B1s)
*   **ORM:** Prisma
*   **In-Memory Cache:** Azure Cache for Redis (Basic C0) (for active game state/matchmaking)
*   **Authentication:** Azure AD B2C or Custom JWT (Decision needed)
*   **Storage (Optional):** Azure Blob Storage (for game history, assets if needed)
*   **Monitoring:** Azure Application Insights (Free tier)
*   **CI/CD:** GitHub Actions (Integrated with Azure Static Web Apps, potentially separate workflow for backend)

---

## Detailed Migration Plan (Azure-Oriented)

### Phase 0: Foundation & Azure Setup

1.  **Project Setup (Local/IDX):**
    *   Create a monorepo structure (e.g., using `npm workspaces`, `pnpm workspaces`, or `Nx`) for `frontend` and `backend`.
    *   Initialize Node.js backend (TypeScript, Express/Fastify, ESLint/Prettier).
    *   Initialize React frontend (Vite/CRA + TypeScript). Install Tailwind CSS.
    *   Set up basic `tsconfig.json` files.
2.  **Azure Resource Provisioning:**
    *   Set up an Azure subscription and resource group.
    *   Provision Azure Static Web Apps instance.
    *   Provision Azure Container Apps environment and a Container App definition (initially basic). *Alternatively, provision an Azure App Service plan (Linux B1).*
    *   Provision Azure Database for PostgreSQL Flexible Server (Burstable B1s).
    *   Provision Azure Cache for Redis (Basic C0 instance).
    *   Provision Azure Blob Storage account (if needed).
    *   Provision Azure Application Insights instance.
    *   **(Decision Dependent)** If using Azure AD B2C, configure a tenant.
3.  **Backend Dependencies:**
    *   Install: `express`/`fastify`, `typescript`, `ts-node`/`tsx`, `socket.io`, `ioredis`, `prisma`, `@prisma/client`, `@azure/identity`, `applicationinsights`, `jsonwebtoken` (if custom JWT).
4.  **Frontend Dependencies:**
    *   Install: `react`, `react-dom`, `typescript`, `tailwindcss`, `socket.io-client`, state management (`zustand` or Context), routing (`react-router-dom`).
    *   **(Decision Dependent)** If Azure AD B2C: `@azure/msal-browser`, `@azure/msal-react`.
5.  **Basic Connectivity & Configuration:**
    *   Setup basic servers (Express/Fastify, Socket.IO).
    *   Configure Prisma (connection string via env var/Key Vault).
    *   Configure Redis client.
    *   Configure Application Insights SDK (backend).
    *   Develop basic `Dockerfile` for the backend.
6.  **Initial CI/CD Setup:**
    *   Configure GitHub Actions for Azure Static Web Apps (frontend).
    *   Set up separate GitHub Actions workflow for backend Docker build, push (ACR/Docker Hub), and deploy to Azure Container Apps/App Service.
7.  **AI Player Implementation:**
    *   Define AI player representation in database (e.g., `Player.isAI` flag).
    *   Update `GameService.createNewGame` to optionally add AI players.
    *   Create `AIService` to encapsulate AI decision-making logic.
    *   Implement basic AI profiles:
        *   **Cautious Farmer (Beginner AI):**
            *   Goal: Avoid debt, maintain positive cash flow.
            *   Behavior:
                *   Prioritizes paying off loans quickly.
                *   Hesitates to buy expensive assets unless cash reserves are very high.
                *   May prefer leasing ridges over buying farms initially.
                *   Less likely to use OTB cards that require significant cash outlay.
                *   Focuses on steady, low-risk income.
        *   **Expansionist (Intermediate AI):**
            *   Goal: Acquire assets rapidly to maximize potential income.
            *   Behavior:
                *   Buys/leases assets whenever affordable, even if it requires taking loans.
                *   Aggressively uses OTB cards to acquire desirable farms/ridges.
                *   Focuses on maximizing harvest potential (acquiring cows, tractors, using modifiers if possible).
                *   Might neglect loan payments slightly in favor of expansion.
        *   **Gambler (Intermediate/Advanced AI):**
            *   Goal: Leverage cards and timing for big gains.
            *   Behavior:
                *   Prioritizes drawing Farmer's Fate cards.
                *   Willing to take risks based on potential card outcomes.
                *   Makes decisions based more on potential high rewards than immediate stability.
                *   Might strategically land on certain tiles to draw cards or trigger specific effects.
        *   **Balanced Farmer (Advanced AI):**
            *   Goal: Adapt strategy based on game state and opponents.
            *   Behavior:
                *   Evaluates asset purchases based on ROI and current debt levels.
                *   Pays attention to opponent net worth and might try to hinder the leader.
                *   Balances expansion, debt management, and card play.
                *   Might switch between conservative and aggressive tactics depending on position.
    *   Integrate AI turn execution into `GameLoopService` or `GameService`.
    *   Implementation Considerations:
        *   Decision Logic: Start with rule-based systems, potentially move towards scoring functions.
        *   Representation: AI players as `Player` records with `isAI: true`, no `User` relationship.
        *   Turn Execution: Automatic decision-making process in game loop.

### Phase 1: Backend - Core State & Logic Porting (Rust -> TypeScript)

1.  **Database Schema (Prisma):**
    *   Analyze `models/*.rs`. Define `schema.prisma` (Game, Player, Asset, CardDefinition, etc.).
    *   Run `prisma generate`.
2.  **TypeScript Models/Interfaces:**
    *   Translate Rust structs (`models/*.rs`) to TypeScript interfaces/types.
3.  **Configuration Porting:**
    *   Translate `config.rs` constants to backend config (use environment variables in Azure).
4.  **Core Game Logic Translation:**
    *   Translate core logic from `game/` into TypeScript service modules (e.g., `src/services/game-logic.service.ts`).
5.  **Game State Management Service:**
    *   Create `GameStateService` using Prisma (PostgreSQL) and `ioredis` (Redis) for load/save/cache.
6.  **Initial Game Setup Logic:**
    *   Translate `main.rs::setup_game` logic into `GameSetupService.createNewGame`, using Prisma/Redis.
7.  **AI Player Implementation:**
    *   Define AI player representation in database (e.g., `Player.isAI` flag).
    *   Update `GameService.createNewGame` to optionally add AI players.
    *   Create `AIService` to encapsulate AI decision-making logic.
    *   Implement basic AI profiles:
        *   **Cautious Farmer (Beginner AI):**
            *   Goal: Avoid debt, maintain positive cash flow.
            *   Behavior:
                *   Prioritizes paying off loans quickly.
                *   Hesitates to buy expensive assets unless cash reserves are very high.
                *   May prefer leasing ridges over buying farms initially.
                *   Less likely to use OTB cards that require significant cash outlay.
                *   Focuses on steady, low-risk income.
        *   **Expansionist (Intermediate AI):**
            *   Goal: Acquire assets rapidly to maximize potential income.
            *   Behavior:
                *   Buys/leases assets whenever affordable, even if it requires taking loans.
                *   Aggressively uses OTB cards to acquire desirable farms/ridges.
                *   Focuses on maximizing harvest potential (acquiring cows, tractors, using modifiers if possible).
                *   Might neglect loan payments slightly in favor of expansion.
        *   **Gambler (Intermediate/Advanced AI):**
            *   Goal: Leverage cards and timing for big gains.
            *   Behavior:
                *   Prioritizes drawing Farmer's Fate cards.
                *   Willing to take risks based on potential card outcomes.
                *   Makes decisions based more on potential high rewards than immediate stability.
                *   Might strategically land on certain tiles to draw cards or trigger specific effects.
        *   **Balanced Farmer (Advanced AI):**
            *   Goal: Adapt strategy based on game state and opponents.
            *   Behavior:
                *   Evaluates asset purchases based on ROI and current debt levels.
                *   Pays attention to opponent net worth and might try to hinder the leader.
                *   Balances expansion, debt management, and card play.
                *   Might switch between conservative and aggressive tactics depending on position.
    *   Integrate AI turn execution into `GameLoopService` or `GameService`.
    *   Implementation Considerations:
        *   Decision Logic: Start with rule-based systems, potentially move towards scoring functions.
        *   Representation: AI players as `Player` records with `isAI: true`, no `User` relationship.
        *   Turn Execution: Automatic decision-making process in game loop.

### Phase 2: Backend - Card System & Effects Porting (Rust -> TypeScript)

1.  **Card Definitions:**
    *   Analyze `cards/` and `original_game_content/`. Store definitions in PostgreSQL `CardDefinition` table. Create population script.
2.  **Deck Management Logic:**
    *   Translate `cards/deck.rs` logic into a `DeckService` using Prisma.
3.  **Card Effect Implementation:**
    *   Analyze `models/effects.rs`. Implement effect execution logic, translating Rust `GameEffect` to TypeScript functions modifying state via `GameStateService`.
4.  **Payment Handling for Card Effects:**
    *   Implement robust payment handling system (`_handlePlayerPayment` in `GameService`).
    *   Handle insufficient funds scenarios with loan options.
    *   Design pattern for payment failure (still discard card if payment fails).
    *   Implement transaction-based updates for atomicity.
    *   Add comprehensive test cases for payment scenarios.
5.  **Initial Card Dealing:**
    *   Integrate card drawing into `GameSetupService.createNewGame`.

### Phase 3: Real-time Layer (Socket.IO on Azure Container Apps/App Service)

1.  **Define WebSocket Events:** Define client-server event contracts (`joinGame`, `playerAction`, `gameStateUpdate`, etc.).
2.  **Backend Socket Handling:**
    *   Implement Socket.IO listeners, rooms.
    *   Validate actions, update state (`GameStateService`), broadcast `gameStateUpdate`.
    *   Log events to Azure Application Insights.
    *   Ensure Azure hosting configuration allows WebSockets and configure scaling.
3.  **Frontend Socket Handling:** Implement Socket.IO emitters/listeners in React.

### Phase 4: Frontend Implementation (React on Azure Static Web Apps)

1.  **Core UI Components:** Translate TUI concepts (`ui/widgets/`) to React components (`GameBoard`, `Scoreboard`, etc.).
2.  **Client State Management (Zustand/Context):** Manage state from WebSockets.
3.  **Connecting UI to WebSockets:** Wire UI interactions to Socket.IO.
4.  **Styling:** Apply Tailwind CSS.
5.  **Azure Static Web Apps Integration:** Ensure frontend routing works with SWA fallback.

### Phase 5: Authentication & Game Management API

1.  **Authentication Integration:**
    *   **(Decision Needed):** Implement Azure AD B2C (MSAL) or custom JWT (`jsonwebtoken`).
    *   Protect backend API/WebSockets. Associate users with `Player` records.
2.  **Game Management API Endpoints:**
    *   Create REST API in Node.js backend (Container Apps/App Service) for `POST /api/games`, `GET /api/games`, etc.
3.  **Frontend Integration:** Implement login/signup UI and game management views.

### Phase 6: Testing, Polishing, Deployment & Monitoring

1.  **Testing:** Unit, integration (backend API, WebSockets), component (frontend), E2E tests.
2.  **UI/UX Polishing:** Refine UI, add loading/error states.
3.  **Deployment:**
    *   Utilize Azure Static Web Apps CI/CD for frontend.
    *   Refine backend GitHub Actions workflow (Docker build/push, deploy to Container Apps/App Service, `prisma migrate deploy`).
4.  **Monitoring:**
    *   Leverage Azure Application Insights for backend/frontend monitoring and alerts.
5.  **Storage Usage (If Applicable):**
    *   Implement Azure Blob Storage read/write if needed.

---

## Addressing Open Questions (`techstacksummary.md`)

*   **Auth (Azure AD B2C vs. Custom JWT):** Decision required for Phase 5 implementation.
*   **Compute (Container Apps vs. App Service):** Plan defaults to Container Apps. Changes require updates in Phase 0 and 6.
*   **Redis Usage (Transient State vs. Matchmaking/Pub-Sub):** Plan assumes transient state. Broader usage requires adjustments in Phase 1 and 3 (e.g., Socket.IO Redis adapter).

## Future Enhancements & Refinements

*   [ ] Explore alternative game board visualization methods (e.g., SVG, Canvas) for better handling of non-rectangular layouts.
*   [ ] Add more sophisticated AI profiles and decision-making logic.
*   [ ] Implement chat functionality within the game.
*   [ ] Enhance animations and visual feedback for game actions.
*   [ ] Consider mobile responsiveness.
*   [ ] Refactor global variables/maps in `main.ts` into proper NestJS providers/modules.
*   [ ] Explore alternative game board visualization methods (e.g., SVG, Canvas) for better handling of non-rectangular layouts.

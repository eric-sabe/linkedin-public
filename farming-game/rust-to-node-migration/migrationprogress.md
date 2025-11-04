# Migration Progress

## Phase 0: Foundational Setup

**Task 1: Project Setup (Local/IDX) - COMPLETE**

*   Initialized monorepo using npm workspaces.
*   Created backend project (`packages/backend`) with Node.js, Express, TypeScript.
    *   Installed core dependencies: `express`, `socket.io`, `ioredis`, `prisma`, `@prisma/client`, `@azure/identity`, `applicationinsights`, `jsonwebtoken`.
    *   Installed dev dependencies: `typescript`, `@types/node`, `@types/express`, `ts-node`, `nodemon`.
    *   Configured `tsconfig.json`.
*   Created frontend project (`packages/frontend`) using Vite, React, and TypeScript.
    *   Installed core dependencies: `react`, `react-dom`, `socket.io-client`.
    *   Installed and configured Tailwind CSS (`tailwindcss`, `postcss`, `autoprefixer`).
    *   Added Tailwind directives to `src/index.css`.

**Recommended Git Commit Message:**
```
feat: initialize monorepo with backend and frontend projects

- Set up npm workspaces for monorepo structure.
- Initialized Node.js/Express backend with TypeScript and core dependencies.
- Initialized React frontend with Vite, TypeScript, Tailwind CSS, and Socket.IO client.
- Completed Phase 0, Tasks 1-4 of migration plan.
```

**Task 2: Azure Resource Provisioning - COMPLETE (via IaC)**

*   Created `infrastructure/main.bicep` file.
*   Defined parameters for resource names, location, admin credentials.
*   Defined Bicep resources for:
    *   Log Analytics Workspace
    *   Application Insights
    *   Container Apps Environment
    *   Container App (with placeholder image)
    *   Static Web App
    *   PostgreSQL Flexible Server (Burstable B1s) + Database
    *   Azure Cache for Redis (Basic C0)
    *   Azure Storage Account
*   Defined outputs for key resource identifiers and connection details.

**Recommended Git Commit Message:**
```
feat(infra): define azure resources using bicep

- Create main.bicep file in /infrastructure.
- Define parameters, tags, and target scope.
- Add resource definitions for Log Analytics, App Insights, Container Apps (Env + App), Static Web App, PostgreSQL, Redis, and Storage Account based on migration plan.
- Include outputs for essential resource properties.
- Completes Phase 0, Task 2 of the migration plan using IaC.
```

**Task 5: Basic Connectivity & Configuration - COMPLETE**

*   Created basic Express + Socket.IO server in `packages/backend/src/server.ts`.
*   Initialized Prisma (`npx prisma init`) in `packages/backend`.
*   Set `provider = "postgresql"` in `packages/backend/prisma/schema.prisma`.
*   Added `DATABASE_URL` placeholder to `packages/backend/.env`.
*   Added `REDIS_HOST`, `REDIS_PORT`, `REDIS_PASSWORD` placeholders to `packages/backend/.env`.
*   Created Redis client configuration in `packages/backend/src/config/redis.ts`.
*   Added `APPLICATIONINSIGHTS_CONNECTION_STRING` placeholder to `packages/backend/.env`.
*   Created Application Insights SDK configuration in `packages/backend/src/config/appInsights.ts`.
*   Ensured App Insights initializes by importing config into `server.ts`.
*   Added `build`, `start`, `prisma:generate`, and `dev` scripts to `packages/backend/package.json`.
*   Created `tsconfig.json` for backend compilation.
*   Created `Dockerfile` in `packages/backend` for containerization.
*   Created `.dockerignore` in `packages/backend`.

**Recommended Git Commit Message:**
```
"feat(backend): configure basic connectivity and docker setup

- Add basic Express/Socket.IO server setup.
- Configure Prisma, Redis, and App Insights with placeholders.
- Create Dockerfile and .dockerignore for backend deployment.
- Add build/start scripts and tsconfig for backend.
- Completes Phase 0, Task 5 of the migration plan."
```

**Task 6: Initial CI/CD Setup & Infrastructure Update - COMPLETE**

*   **Infrastructure (`infrastructure/main.bicep`):**
    *   Added `Azure Container Registry (ACR)` resource (`Microsoft.ContainerRegistry/registries`).
    *   Added parameters: `acrName`, `containerImageName`, `servicePrincipalObjectId`.
    *   Configured `Container App` registry settings to pull from ACR (using admin user initially).
    *   Enabled System Assigned Managed Identity for `Container App`.
    *   Added Role Assignment: `Container App` Managed Identity granted `AcrPull` role on ACR.
    *   Added Role Assignment: CI/CD Service Principal (via `servicePrincipalObjectId` parameter) granted `AcrPush` role on ACR.
    *   Updated `Container App` ingress `targetPort` to 3001.
    *   Added outputs for ACR details (`ACR_LOGIN_SERVER`, `ACR_NAME`, `ACR_ADMIN_USERNAME`, `ACR_ADMIN_PASSWORD`).
*   **CI/CD Workflows:**
    *   Created frontend deployment workflow: `.github/workflows/azure-static-web-apps.yml`.
        *   Builds `packages/frontend` and deploys to Azure Static Web Apps using `AZURE_STATIC_WEB_APPS_API_TOKEN` secret.
    *   Created backend deployment workflow: `.github/workflows/backend-deploy.yml`.
        *   Builds Docker image from `packages/backend/Dockerfile`.
        *   Pushes image to ACR (requires `REGISTRY_*` secrets).
        *   Deploys image to Azure Container Apps (requires `AZURE_CREDENTIALS`, app/RG names, and service secrets like `DATABASE_URL`, `REDIS_*`, `APPLICATIONINSIGHTS_CONNECTION_STRING`).

**Note:** Infrastructure deployment requires pre-creation of a Service Principal and providing its Object ID (`servicePrincipalObjectId`) as a parameter. Workflows require configuration of GitHub secrets.

**Verification:** Successfully deployed Bicep template to Azure resource group `rg-farmgame-dev`, provisioning all defined resources.

**Recommended Git Commit Message:**
```
"ci: add initial workflows and update infra for CI/CD

- Create GitHub Actions workflows for frontend (SWA) and backend (ACA) deployment.
- Add Azure Container Registry (ACR) resource to main.bicep.
- Configure Container App to use ACR and enable Managed Identity.
- Add AcrPull (for Container App MI) and AcrPush (for CI/CD SP) role assignments to Bicep.
- Completes Phase 0, Task 6 including infra updates for CI/CD.
- Requires SP creation and GitHub secrets configuration."
```

**Task 9: Turn Management and Tile Actions Implementation - COMPLETE**

*   Turn Management:
    *   Implemented turn order shuffling at year end using Fisher-Yates algorithm
    *   Added proper turn progression with player state updates
    *   Implemented "miss next turn" effect handling
    *   Added turn advancement after bankruptcy
    *   Implemented turn-based action validation
*   Tile-based Actions:
    *   Implemented comprehensive tile effect system
    *   Added tile effect handling in game loop
    *   Implemented tile-based payments and actions
    *   Added proper test coverage for tile effects
    *   Integrated tile effects with player state updates

**Recommended Git Commit Message:**
```
docs: update migration progress for turn management and tile actions

- Mark turn management implementation as complete
- Mark tile-based actions implementation as complete
- Document implemented features in both systems
```

## Phase 1: Backend - Core State & Logic Porting (Rust -> TypeScript)

**Task 1: Database Schema (Prisma) - COMPLETE**
*   Analyzed `models/*.rs`. Defined `schema.prisma` (Game, Player, Asset, CardDefinition, etc.).
*   Ran `prisma generate`.
*   [ ] Revisit `maxPlayers` requirement and define `BoardTile` model in `schema.prisma` (Ref: `schema.prisma` TODOs).

**Task 2: TypeScript Models/Interfaces - COMPLETE**
*   Utilizing Prisma-generated TypeScript types from `../generated/prisma`.

**Task 3: Configuration Porting - COMPLETE**
*   Translated `config.rs` constants to backend config (`packages/backend/src/config/game.config.ts`).
*   Defined card catalog in `packages/backend/src/config/cards.catalog.ts`.
*   Using environment variables in Azure (placeholders in `.env`).
*   [ ] Implement loading base asset values (`AssetService` Placeholder).

**Task 4: Core Game Logic Translation - MOSTLY COMPLETE / REFINEMENT NEEDED**
*   Core logic translation completed in TypeScript service modules (e.g., `game.service.ts`, `game-loop.service.ts`, `tile-effect.service.ts`, `harvest.service.ts`). Covers turn loop, movement, tile effects, harvest, payments, and card effects.
*   [ ] Refine core logic implementation based on remaining TODOs:
    *   [x] Handle insufficient funds (`GameService`)
        *   [x] Implemented `_handlePlayerPayment` method that checks for sufficient funds
        *   [x] Added automatic loan taking when players have insufficient cash
        *   [x] Set up failure pattern that still discards cards when payment fails
        *   [x] Implemented transaction-based updates for payment handling
        *   [x] Added comprehensive test suite for various payment scenarios
    *   [ ] AI Player setup/DB record (`GameService`)
    *   [ ] Deal initial cards for joining player (`GameService`)
    *   [x] Trigger tile effects after moving (`GameService`)
    *   [x] Add end-of-year logic (interest, harvest checks?) (`GameService`, `TileEffectService` modifier clearing)
        *   [x] Implemented side job payment processing when passing Go
        *   [x] Added temporary modifier clearing using `clearEndOfYearModifiers` function
        *   [x] Implemented interest payment calculations (10% on outstanding debt)
        *   [x] Set up harvest calculations with yield tables for all crop types
        *   [x] Created phase transitions (EarlySummer, LateSummer, EndOfYear)
    *   [x] Add movement calculation logic (`BoardService`)
    *   [x] Refine Harvest logic (reset multipliers, temporary modifiers, bonuses, asset updates) (`HarvestService`)
    *   [ ] Refine Bankruptcy/Auction logic (cost basis, ridge auction rules) (`BankruptcyService`)
    *   [x] Implement Player Asset add/remove/transfer logic (`PlayerService` Placeholders)
    *   [ ] Ensure correct PlayerAsset income updates (`TileEffectService`, `GameService`)
    *   [ ] Implement scoreboard updates (`GameLoopService`)
    *   [ ] Implement game phase changes & end condition checks (`GameLoopService`)
    *   [ ] Shuffle turn order (`GameService`)
    *   [x] Check ridge lease availability (`GameService`)
    *   [ ] Add tile-based actions (`GameService`)
    *   [ ] Refine turn management (`GameService`)

**Task 5: Game State Management Service - REVISED / DEFERRED**
*   Game state loading/saving/updating implemented using Prisma (PostgreSQL) across various services.
*   No dedicated `GameStateService`. Planned Redis usage for active state caching appears **DEFERRED** (decision made to proceed without it for now).
*   [ ] Decide on and implement board state persistence strategy (`GameService` TODO).

**Task 6: Initial Game Setup Logic - COMPLETE**
*   Translated `main.rs::setup_game` logic into `GameService.createNewGame`, using Prisma/Postgres. Initializes Game, Players, Assets, Decks, Ridges.

**Task 7: AI Player Implementation - IN PROGRESS**
*   [x] Define AI player representation in database:
    *   [x] Add `isAI: boolean` to `Player` model in `schema.prisma`.
    *   [x] Add `aiProfile: AIProfile?` enum to `Player` model for profile type.
    *   [x] Define `AIProfile` enum with types: CautiousFarmer, Expansionist, Gambler, BalancedFarmer.
    *   [x] Run `prisma generate` and update test files to include new fields.
*   [x] Update `GameService.createNewGame`:
    *   [x] Add `numAI: number` parameter (default 0).
    *   [x] Add validation for total players (human + AI).
    *   [x] Create AI players with unique names and colors.
    *   [x] Deal initial assets and cards to AI players.
*   [x] Create `AIService` (`packages/backend/src/services/ai.service.ts`):
    *   [x] Define `AIActionType` enum and `AIAction` interface.
    *   [x] Create base `determineNextAction` method.
    *   [x] Implement profile-specific decision logic:
        *   [x] **Cautious Farmer:**
            *   [x] Implement loan payment priority.
            *   [x] Add cash reserve thresholds for asset purchases.
            *   [x] Add ridge lease preference logic.
            *   [x] Implement conservative OTB card usage.
        *   [x] **Expansionist:**
            *   [x] Implement aggressive asset acquisition.
            *   [x] Add loan tolerance thresholds.
            *   [x] Implement OTB card usage strategy.
            *   [x] Add harvest potential maximization logic.
        *   [x] **Gambler:**
            *   [x] Implement card draw priority.
            *   [x] Add risk-based decision making.
            *   [x] Implement strategic tile targeting.
            *   [x] Add high-reward action prioritization.
        *   [x] **Balanced Farmer:**
            *   [x] Implement ROI-based asset evaluation.
            *   [x] Add opponent analysis and counter-strategies.
            *   [x] Implement adaptive strategy switching.
            *   [x] Add position-based tactic selection.
*   [x] Implement AI execution in `GameLoopService`:
    *   [x] Add `executeAITurn` method to handle AI player turns.
    *   [x] Process different AI action types (roll dice, pay loan, buy asset, etc.).
    *   [x] Handle game state updates after AI actions.
    *   [x] Ensure proper turn advancement after AI moves.
*   [x] Integrate AI execution into game loop:
    *   [x] Verify `GameService.advanceTurn` handles AI players.
    *   [x] Ensure AI actions are executed automatically.
    *   [x] Implement proper logging of AI decisions and actions.
*   [x] Add AI Player UI/UX Flow:
    *   [x] Create frontend UI components for configuring AI players:
        *   [x] Add AI player count selector in game creation form.
        *   [x] Implement AI profile selection (optional, otherwise randomized).
        *   [x] Add UI indicators for AI players during gameplay.
    *   [x] Update backend to support AI configuration:
        *   [x] Extend game creation API to accept AI configuration.
        *   [x] Add validation for AI parameters.
        *   [x] Implement API response with created AI player details.
    *   [x] Implement AI profile management:
        *   [x] Add ability for game owner to change AI profiles during gameplay.
        *   [x] Display visual indicators of AI profile types.
        *   [x] Update backend endpoint for profile changes.
*   [x] Add unit tests:
    *   [x] Test AI player creation in `createNewGame`.
    *   [x] Test AI decision making for each profile.
    *   [x] Test AI turn execution and progression.
    *   [x] Test game flow with mixed human and AI players.

**AI Player Creation and Game Flow**

The following diagrams outline the flow for creating and interacting with AI players:

1. **AI Player Creation Flow**:
   ```
   [User Interface] → Create Game Form → Set numAI → (Optional) Configure AI Profiles
        ↓
   [API] → POST /games → GameService.createNewGame(userId, numAI)
        ↓
   [Database] → Create Game → Create Human Player → Create AI Players → Initialize Game State
   ```

2. **Game Turn Flow with AI Players**:
   ```
   [Human Player Turn] → Take Actions → End Turn → GameService.advanceTurn()
        ↓
   [Turn Advancement Logic] → Check if next player is AI → If yes, initiate AI Turn
        ↓
   [AI Turn] → GameLoopService.executeAITurn(gameId, aiPlayerId)
        ↓
   [AIService] → determineNextAction(aiPlayer, game) → Return AIAction
        ↓
   [GameLoopService] → Process AIAction (e.g., handlePlayerTurn for roll, update DB for others)
        ↓
   [Turn Advancement Logic] → Update Game State → Advance to next player (Human or AI)
   ```

**Recommended Git Commit Message:**
```
feat(ai): implement AI player system with profiles and automatic turn execution

- Add AI profile to Player schema with four AI types
- Update GameService.createNewGame to create AI players during game setup
- Implement AI decision logic in AIService with different strategies per profile
- Add GameLoopService.executeAITurn to process AI turns automatically
- Update GameService.advanceTurn to detect and trigger AI turns
- Create outlined task list for AI player UI/UX flow
- Progress on Phase 1, Task 7 of migration plan (AI player implementation)
```

## Phase 1 (Continued): High Priority Implementation Tasks

**Task 1: Core Game Logic Refinements - IN PROGRESS**

*   [x] Implement scoreboard updates in GameLoopService:
    *   Added `calculatePlayerNetWorth` method to compute total player value
    *   Integrated scoreboard updates into turn processing
    *   Added net worth tracking in player model
*   [x] Add game phase change logic and end condition checks:
    *   Implemented `determineNextPhase` method for phase transitions
    *   Added phase mapping configuration in `phase.mapping.ts`
    *   Integrated phase changes with turn advancement
    *   Added end condition checks for winning net worth, max turns, and player elimination
*   [ ] Complete bankruptcy/auction logic:
        *   [x] Basic bankruptcy detection and loan system
        *   [x] Asset auction implementation with AI bidding
        *   [x] Add cost basis tracking for auctioned assets (winner assigned asset base value)
        *   [x] Implement ridge auction rules
        *   [x] Add refinancing penalties (20% fee) for emergency loans
        *   [x] Ensure minimum $5,000 operating cash (auction stops when threshold met)
        *   [x] Implement specific Cow auction rules (10-head blocks, $5k value, 20 farm cow limit for bidders, decrement bankrupt player)
        *   [x] Implement Grain/Hay (10-acre) & Fruit (5-acre) block auction rules (decrement bankrupt player)
        *   [x] Unsold blocks are retained by the bankrupt player
*   [ ] Implement turn order shuffling:
    *   [x] Add shuffle logic in GameService (end of year)
    *   [x] Integrate with game phase changes (part of end-of-year block)
    *   [x] Handle AI player ordering (included in shuffle)

**Task 2: Real-time Layer Implementation - IN PROGRESS**

*   [x] Define WebSocket events and payloads
*   [x] Set up basic Socket.IO server
*   [ ] Complete remaining game action handlers:
    *   [x] Player card plays and effects (`applyCard` added)
    *   [ ] Player movement and dice rolls (Handler exists, needs frontend integration/testing)
    *   [-] Asset purchases (`buyAsset` handler removed - only occurs via card effects)
    *   [ ] Asset sales (Deferred - Requires Player-to-Player Sale feature)
    *   [ ] Bankruptcy and auction events (If needed for UI interaction)
*   [ ] Implement game state synchronization:
    *   [x] Basic synchronization implemented (sending full game state)
    *   [ ] Add state diff calculation (Deferred - Future Optimization)
    *   [ ] Set up efficient update broadcasting (Partially complete - uses full state)
    *   [ ] Handle reconnection scenarios (Needs specific handling)
*   [ ] Add error handling and recovery:
    *   [ ] Implement retry mechanisms
    *   [ ] Add state reconciliation
    *   [ ] Handle disconnection scenarios

**Task 3: Frontend Core Components - IN PROGRESS**

*   [ ] Create GameBoard component:
    *   [ ] Implement tile rendering
    *   [ ] Add player position tracking
    *   [ ] Create movement animations
*   [ ] Build Scoreboard component:
    *   [ ] Display player stats
    *   [ ] Show net worth rankings
    *   [ ] Add asset summaries
*   [ ] Implement ActionPanel:
    *   [ ] Add dice roll controls
    *   [ ] Create asset management UI
    *   [ ] Add card play interface
*   [ ] Create CardHand component:
    *   [ ] Display current cards
    *   [ ] Add card play functionality
    *   [ ] Show card effects

**Recommended Git Commit Message:**
```
feat(game): implement core game logic refinements

- Add scoreboard updates with net worth calculation
- Implement game phase transitions and end conditions
- Add basic bankruptcy and auction system
- Set up real-time layer foundation with Socket.IO
- Create initial frontend component structure
- Progress on Phase 1 high-priority tasks
```

**Task 7: Frontend Core Components Implementation - COMPLETE**

*   Enhanced `GameBoard.tsx` with:
    *   Grid layout for game board visualization
    *   Player position tracking with color-coded markers
    *   Phase-based tile coloring
    *   Tile information display with hover tooltips
*   Implemented comprehensive `Scoreboard.tsx` with:
    *   Player stats display (cash, net worth, position)
    *   AI profile management for game owners
    *   Turn order indication
    *   Player status tracking (active/disconnected)
*   Created robust `ActionPanel.tsx` with:
    *   All game actions (roll dice, pay loan, play cards, etc.)
    *   Modal system for complex actions
    *   State management integration
    *   Clear action availability indicators
*   Enhanced `PlayerHand.tsx` with:
    *   Support for all card types (OTB, Lease, Event, Asset)
    *   Visual separation of OTB and playable cards
    *   Color-coded card type indicators
    *   Action availability indicators
    *   Improved card visualization with hover effects

**Recommended Git Commit Message:**
```
feat(frontend): implement core game UI components

- Enhance GameBoard with grid layout and player tracking
- Create comprehensive Scoreboard with player stats and AI management
- Implement ActionPanel with modal system for game actions
- Enhance PlayerHand with support for all card types and improved visualization
- Complete frontend core components implementation for gameplay testing
```

**Task 8: Bankruptcy and Auction Events Implementation - COMPLETE**

*   Added new socket event types in `shared-types`:
    *   Client events: `placeBid`, `declareBankruptcy`
    *   Server events: `auctionStarted`, `auctionUpdate`, `auctionEnded`, `bankruptcyDeclared`, `bankruptcyProcessed`
*   Enhanced `socketService.ts` with:
    *   New event handlers for auction and bankruptcy events
    *   State management for auction progress and bankruptcy status
    *   Client-side validation for bids and bankruptcy actions
*   Updated game types to support:
    *   Auction state tracking in `GameState`
    *   Bankruptcy and elimination status in `PlayerPublic`
    *   New `Auction` interface for tracking auction details
*   Created new `AuctionPanel.tsx` component with:
    *   Real-time auction display with current bid and time remaining
    *   Bid placement interface with validation
    *   Bankruptcy declaration and confirmation UI
    *   Integration with game state and socket service

**Recommended Git Commit Message:**
```
feat(frontend): implement bankruptcy and auction event handlers

- Add socket event types for auctions and bankruptcy
- Enhance socket service with auction/bankruptcy event handlers
- Update game types to support auction state and bankruptcy status
- Create AuctionPanel component for auction participation and bankruptcy declaration
- Complete frontend implementation of bankruptcy/auction game mechanics
```

## Phase 2: Backend - Card System & Effects Porting (Rust -> TypeScript)

**Task 1: Card Definitions - COMPLETE**
*   Analyzed `cards/` and `original_game_content/`.
*   Static card data defined in `packages/backend/src/config/cards.catalog.ts`.
*   `DeckService.ensureCardDefinitionsExist` populates/updates PostgreSQL `CardDefinition` table.

**Task 2: Deck Management Logic - COMPLETE**
*   Translated `cards/deck.rs` logic into a `DeckService` using Prisma. Handles deck initialization, draw, discard, shuffling (including OTB).

**Task 3: Card Effect Implementation - COMPLETE**
*   Analyzed `models/effects.rs`.
*   `GameService.applyCardEffect` implements execution logic for many card effects (Income, Expense, Conditional Payments, Movement, Asset changes, Draw, etc.), updating state via Prisma calls within services.
*   OTB/Lease effects correctly separated into `exerciseOptionToBuyAsset` / `exerciseOptionToLeaseRidge`.
*   Implemented `SlaughterCowsWithoutCompensation` effect.

**Task 4: Initial Card Dealing - COMPLETE**
*   `GameService.createNewGame` now deals 2 Farmer's Fate cards to each player's hand upon game creation.

## Phase 3: Real-time Layer (Socket.IO on Azure Container Apps/App Service)

**Task 1: Define WebSocket Events - COMPLETE**
*   Created dedicated `@farming_game/shared-types` package.
*   Defined client-server event constants (e.g., `CLIENT_EVENT_JOIN_GAME`, `SERVER_EVENT_GAME_STATE_UPDATE`) in `packages/shared-types/src/socket-events.ts`.
*   Defined associated payload interfaces (e.g., `ClientJoinGamePayload`, `ServerGameStateUpdatePayload`) using shared types.
*   Defined core shared data types (`GameState`, `PlayerPublic`, `BoardTile`, etc.) in `packages/shared-types/src/game-types.ts`.
*   Configured workspaces (npm, pnpm) to link shared package.

**Task 2: Backend Socket Handling - MOSTLY COMPLETE**
*   Basic Socket.IO server setup in `

## Next Steps and Priorities

Based on the current progress and the migration plan, here are the recommended next actions:

1. **Refine Core Game Logic (High Priority)**
   * Address remaining TODOs in Task 4:
     * Add end-of-year logic (interest, harvest checks)
     * Refine Bankruptcy/Auction logic
     * Implement scoreboard updates
     * Add tile-based actions
     * Refine turn management
   * This ensures robust game mechanics before moving to final phases

2. **Complete Phase 3: Real-time Layer (Medium Priority)**
   * Finish WebSocket implementation
   * Ensure proper real-time updates across clients
   * Implement game state synchronization
   * This enables the full multiplayer experience

3. **Move to Phase 4: Frontend (Medium-High Priority)**
   * Complete React component implementation
   * Implement responsive design for mobile/desktop
   * Add animations and transitions for better UX
   * This delivers the final user-facing experience

**Recommended Git Commit Message for Current Progress:**
```
test(ai): implement comprehensive AI player tests

- Add AIService unit tests for each AI profile type's decision making
- Create tests for AI player creation with various profiles
- Add GameLoopService tests for AI turn execution
- Implement mixed player game flow tests with human/AI progression
- Complete Phase 1, Task 7 testing requirements
```

**Task 8: Linter and Type Errors - COMPLETE**
*   [x] Fixed multiple linter errors in `packages/backend/src/services/game-loop.service.ts`:
    *   [x] Resolved duplicate and incorrect Prisma type imports by separating `PrismaClient` (@prisma/client) from generated types (`../generated/prisma`).
    *   [x] Corrected usage of `GameStatus` enum.
    *   [x] Added explicit types to lambda parameters (`p`, `asset`, `player`) to fix implicit `any` errors.
    *   [x] Added checks for potentially `undefined` player objects retrieved using `.find()`.


    **This was randomly put in the wrong file**
    ## AI Implementation
- [x] Create AI service with decision logic
- [x] Implement AI player profiles (Cautious Farmer, Expansionist, Gambler, Balanced Farmer)
- [x] Add AI profile field to Player model
- [x] Implement AI turn execution
- [x] Add AI player creation to game setup
- [x] Test AI gameplay scenarios 
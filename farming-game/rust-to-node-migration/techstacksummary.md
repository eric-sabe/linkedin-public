## ✅ Tech Stack Summary (Azure-Oriented, $140/month Budget)

### ✅ Tech Choices

- **Frontend Hosting**
  - **Azure Static Web Apps** (React + TypeScript)
  - Includes global CDN, SSL, CI/CD from GitHub
  - **Cost**: Free or $9/month

- **Backend (Game Logic & WebSocket API)**
  - **Primary**: Azure Container Apps (Dockerized Node.js + Socket.IO)
  - **Alternate**: Azure App Service (Linux, B1 tier)
  - **Cost**: ~$13–30/month
  - ✅ Recommendation: **Azure Container Apps**

- **Database**
  - **PostgreSQL Flexible Server (Burstable B1s)**
  - **ORM**: Prisma
  - **Cost**: ~$16–20/month

- **In-Memory Store**
  - **Azure Cache for Redis (Basic C0)**
  - Used for active game state or matchmaking
  - **Cost**: ~$17/month

- **Authentication**
  - **Azure AD B2C** (Free for up to 50,000 users)
  - ✅ Option: Use **JWT-based custom auth** with `jsonwebtoken`

- **Storage (Game History, Assets)**
  - **Azure Blob Storage**
  - Pay-as-you-go pricing
  - **Cost**: ~$1–3/month

- **Monitoring / Telemetry**
  - **Azure Application Insights**
  - **Free Tier**: 5 GB/month included

---

### ❓ Open Questions

- Will auth be handled via **Azure AD B2C** or **custom JWTs**?
- Is **persistent compute** (App Service) needed, or is **consumption-based scaling** (Container Apps) sufficient?
- Will **Redis** be used **only for transient game state**, or also for **matchmaking/pub-sub**?


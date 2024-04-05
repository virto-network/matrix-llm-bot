This file is a recopilation of information about Virto. Here we have:

1. The local incentives protocol
2. The Kreivo publication on polkadot's forum
3. Some of the proposals that the Virto team has been publishing with information about what we do. 


1. Local Incentives Protocol
Decentralized Marketplace Infrastructure
by Qian Che and Daniel Olano · 2021

1. Introduction
Fiat money gives central banks huge control over the economy, an alerting thought since it is extensively used in all human trading activities. Risks have been observed in a number of countries as hyperinflation resulted from huge increases in the supply of paper money, e.g. currency instability in Venezuela that began in 2016. Since the launch of Bitcoin in 2009, cryptocurrencies are not yet considered as mainstream currency in payment systems in exchange for common goods or services, e.g. a pizza, a painting, a house and so on. To fulfill this demand, Virto Network creates the Local Incentives Protocol(LIP) which defines a secure payment system and means to trade off-chain assets with cryptocurrency by connecting decentralized markets and users, LIP facilitates a better local welfare redistribution through local tax collection from economic activities.

Why? We live in a modern 21st century yet inequality still persists, whether in income, wealth, opportunities or other dimensions. Some of the most effective mechanisms governments have for reducing economic inequalities are taxes, cash and in-kind transfers. However these fiscal policies that help shape more equitable societies play a small role in low-income countries.

UNDP https://unstats.un.org/sdgs/report/2021/goal-10/

Through historical perspective, we observe the limitations of the current government taxing system to resolve economic inequality

Fairness: Tributed local resources are managed by people with different interests
Efficiency:Redundant intermediaries and bureaucratic bodies cost time and money
Transparency: If people can’t track and feel the benefit of the spending process it lowers the trust in the system
Therefore we believe that LIP enables connected decentralized marketplaces to empower autonomous local communities by letting people and local communities be in control to collect and spend local tax in a fair, efficient and transparent way. Virto Network implements LIP as a token-less parachain in the Polkadot network providing Marketplace builders developer friendly API’s to easily build secure & highly scalable decentralized commerce applications so they can focus on solving the real problems.

2. Local Incentives Protocol
Local Incentives Protocol enables people to create stronger communities with aligned interests, to transact securely for goods and services in all kinds of decentralized marketplaces and to receive incentives through local tax collection to solve community problems in a fair, efficient and transparent way. All are enforced by blockchain technology.

2.1 Key actors
Buyers/Customers: Anyone effortlessly joining the network in a trustless way that wants to buy goods or services from any other user.
Sellers/Merchants: More experienced users trading their real world assets and services in exchange for cryptocurrency.
Local community: People sharing a physical space who are incentivized to create communities and unlock existing and future resources which are collected from every trade with a local tax. Communities as DAOs decide to spend resources based on their needs to create powerful autonomous micro-economies
Commercial community: People with a common interest in an economic activity create marketplaces that act like token curated registries where merchants post their products and services. They issue their own token used to bring value and utility to their community.
Union: Communities and Marketplaces can together form unions, as a higher level DAOs to make decision to tackle global level topics.
2.2 Process
The world is divided into "Geo land cells". Each cell represents around 5 km2. When an individual creates an account, they register themselves in the cell of their choosing.

Individuals willing to exchange goods or services have two means of trading, peer-to-peer or via a commercial community.

In P2P trading, people can freely trade as buyer and seller but are subject to a higher cost per transaction.
When trading in in a commercial community, individuals apply to become qualified sellers or buyers that are subject to the higher quality standards of said community.
Therefore, individuals can freely trade with each other or in a regulated marketplace run by the commercial community, both benefiting from the same secure payment system embedded in the protocol, which enables cryptocurrencies as payment method for service and off-chain assets. Trading in a marketplace means the commercial community collects a market fee in their native token as the cost of their service, which encourages new commercial communities to join the system by creating applications to serve the demands of the market.
Commercial communities also play a crucial role in the system as infrastructure providers, with the collected fees they are expected to maintain a minimum required server infrastructure to serve the market they wish to target, to serve larger geographical and more densely populated areas, more infrastructure and support from token holders is necessary, a mechanism that could incentivize the creation of smaller markets instead of bigger ones.

For every trade within the system, a minimal system fee is charged to secure the network and prevent abusive usage. Besides, a certain percent of the traded amount is collected as local tax, which is directly locked in the Geo land cells that could become future local communities. To form a community, individuals from the same cell need to stake together enough resources passing a threshold that indicates their willingness to form the community.
The system allows staking DOT tokens in local pools as they are the native currency of the network to earn rewards coming from the Polkadot relay-chain as well as getting rewards in land tokens.
Land tokens are unique non fungible assets that can be freely traded and collected for use in local and system governance.
Due to the accumulated local tax, people are incentivized to create communities with their peers, bonding assets to unlock existing and future resources that are collected in the cells from trading activity. Besides, the system fee collected also belongs to local communities. These communities will become DAOs and decide to spend resources based on their needs to create powerful autonomous micro-economies. Communities could decide about their own governance system to better facilitate their economy in a dynamic way.

Local and commercial communities can also form unions, a permanent or temporary organization, to collectively solve bigger common goals. For example, all communities in the system can be part of a global union that collects funds to fight climate change.

2.3 Applications
To better serve their users, Commercial Communities often create their own decentralized marketplace applications that connect to LIP for users to directly to interact with their well curated listings.
All sorts of economic activities can take place in the marketplaces. The following are reference applications developed by the core team as a showcase. More applications are expected to be developed by other builders.

Swap.cash: Decentralized on & off ramps, which enables an easy exchange between fiat money and cryptocurrencies
Flea.market: P2P trade of local goods
Go.delivery: Deliveries without intermediaries
3. Infrastructure
Local and Commercial communities are the main key entities in charge of providing infrastructure to the network, although anyone can run the necessary software to to participate, it becomes a requirement for them as they have the economic incentives to do so.

LIP distinct feature is to combine three different kinds of decentralized technology, a Substrate based parachain to be deployed in the Polkadot network, Matrix, the network for secure decentralization communications and Valor, an application runtime and development platform.

3.1 Parachain
A minimalistic blockchain runtime is used only for consensus critical functionality.

Payments: The core functionality of the system, enables secure payments of off-chain assets with a highly configurable escrow-like system that protects users funds until the different parties have come to an agreement.
Communities: DAO management tools that allow Unions, Local and Commercial communities take part in on-chain governance, treasury management, etc.
Network assets: The parachain makes strong usage of inter-chain communication to allow merchants transact with all kinds of assets that might exist elsewhere.

3.2 Matrix
LIP uses the communications protocol in different ways,

In the payment system it provides users with encrypted communication channels to exchange and store sensitive personal information.
For key management and improved user experience, users can use their matrix account and familiar login facilities to interact with the blockchain.
Marketplaces use specialized Matrix rooms and custom signed messages for merchant's listings.
As general purpose message storage for non-consensus-critical data and as layer-two-like scaling platform, Matrix syncs data across a decentralized network of homeservers using rooms where only the involved parties need to store data.
3.3 Valor
A lightweight application runtime that allows composing community developed services that run in a community's infrastructure or fully decentralized in a user's device. One big focus of the runtime is to allow developers to use technologies the are already familiar with(e.g. HTTP based APIs) and have extensive support across all kinds of platforms. Due to the portability of the runtime and keeping applications' state in decentralized storages (i.e. blockchain, Matrix homeservers, IPFS), said APIs can be run unchanged in the user devices, including web browsers, making it an excellent Web 3 development environment.

4. Governance
In its early days, LIP is governed mainly by the founding team which aims to function as a benevolent dictatorship to set up the initial system and maximize overall welfare and efficiency.

The more local and commercial communities are formed, each of them will form their own governance structure and achieve autonomy with a democratic and self-sufficient political economic system. Each community adopts their preferred democratic system based on different choices given by the protocol, being the one-person-one-vote system the default one. The global LIP system is governed by local and commercial communities as well as the founding that is programed to reduce its decision power gradually over time.

5. Economics
Different tokens are used across the system.

Native Token: The parachain doesn't issue its own token, instead it uses Polkadot's DOT as native token, which follows Polkadot Network governance and distribution. We allow DOT staking via communities for participants to get rewards.
Land Token: Land token is a form of hierarchical non-fungible "geo-token" within LIP representing land in the real world. Geo-tokens have different levels representing different varying sizes of land. Local communities are represented with geo-tokens starting at around 5 km2, derived from a H3(Hexagonal hierarchical geospatial indexing system) cell of level 7 resolution. The higher the level, the smaller the represented land size. The minimal unit of a geo-token that can be held and traded by a user is level 13 that represents around 43.9m2. The hierarchical nature of the token means a community(Lv7) holds around 117.6K of geo-token units.
Marketplace Token: Each commercial community that connects to LIP can mint its own token for its operation. They also establish their own governance system and token economics. For every trade in a marketplace, the marketplace token is charged as the market fee and is converted from whichever currency was used for the trade, like a stable-coin, using a local decentralized exchange. A local tax that gets locked within the land cell for local communities to use.
5.1 Land Token
5.1.1 Key Functions
Network Utility: Land token is a tradable NFT token that can be transfered across chains. and also utilized
Local Governance: Land tokens can provide their holders voting rights at local community level and protocol wide voting power.
Staking reward: Land tokens can be minted as reward for staking DOT.
Community creation: Communities are created when enough land token holders within the community cell second the proposal for the community to be formed.
Commercial community support: Land token holders can require deny or approve new businesses from operating in their geographical area.
5.1.2 Minting and Distribution
The total of 117.6K( =76) land tokens of each community is minted starting from time 0. The speed of token minting is rather slow in the beginning, then it speeds up to finally slow down as shown below. In each period, the total staking capital decides the chance for the individual to get the land token, usually staking higher capital means there is a higher chance of a reward until it reaches a certain threshold when the chance of winning no longer increases. The mechanism tries to increase fairness by preventing early birds and whales to take absolute advantage and monopoly the system's governance.

5.2 Marketplace Token
5.2.1 Key Functions
Marketplaces are free to decide how their token is used. Some examples are:

Value accrual: Marketplace token is used to collect fees on every trade.
Community Governance: As a governance token, marketplace tokens provide their holders voting rights within the community.
Transparent marketing and growth: Commercial communities often allocate tokens to be used as rewards to attract more users into their platforms.
5.2.2 Minting and Distribution
Commercial communities are free to design their own token economics as they see fit with the condition that the system's governance controls their token's issuance, in the case of Virto Network the parachain will reserve minimum a 5% of a community's token allocation for itself so other actors in the system can participate in the community's governance and take decisions that benefit everyone in the system.

5.3 Launch on Polkadot as Parathread
Virto Network is the only planned implementation of LIP as we believe the uniqueness of the Land Tokens should be kept(i.e. not two people should own the same land in different systems). Also given its social impact and being designed as a sustainable "token less" chain that inherits the relay chain token(DOT) we believe there is place for the network to be considered a common good chain.
As an alternative, Virto Network will operate as a Parathread paying for its operation using the system fee until it can produce enough value to secure a long-term parachain slot for itself in the future.

2. # Kusama lives and makes creativity* possible - by Virto

author: olanod
date: 1/11/23

Kreivo means creativity and it’s also the name of the parachain the Virto team deployed in Kusama as the winner of auction 102 a couple of months ago.
We will use this parachain to test and integrate the tools focused on UX/DevX that we have developed for the past 3 years and [any future developments](https://github.com/orgs/virto-network/projects/11/views/1) 6, as well as integrating real-world applications that partners in Latin America are excited to turn into Web3 businesses with social impact.

Kreivo [is not a new thing](https://kusama.polkassembly.io/post/1520#48ad945b-02d2-40aa-9961-43c383d9d773) 5, it just took a while to find the right conditions to showcase this bet for **a different kind of parachain**, one that is kick-started by the community and behaves as a common good not having a token and using **KSM as it’s native asset** while remaining sovereign and self-sustainable.

We have been preparing the ground for some years now, with the help of grants and then earlier this year when the Virto team received 6 months support for a [“trial fund”](https://kusama.polkassembly.io/referenda/77) 4 to prove itself as a capable ecosystem agent that can bring innovative solutions to the common problems that affect us. But tools need to be used! with Kreivo we have the playground to experiment and put together the many pieces of our puzzle we’ve been building.

### Kreivo as a product
We are creating a robust and sustainable payments & community management infrastructure for cooperatives(commercial communities) whose users transact with real-life products and services. A standout feature is the ability to lock a small contribution from each payment in the region where the transaction occurred to incentivize the creation of local communities. It is our long term mission to use this technology to fight inequality(specially in Latam).

The Virto team is joining as one more commercial community to provide the infrastructure services of the underlying protocol in a simple manner. Anyone looking to establish Web3 cooperative will see a simple subscription based Solution as a Service platform that is a smash of Discord, Stripe and Wix.

> Notice how I don’t use the term DAO? it’s important for us and the projects we want to attract to exclude blockchain jargon 
> or any related complexity to our communities and their end users, the team has experience building products that use
> blockchain technology that is completely hidden away from the end user.

#### Other use-cases
Kreivo is a great platform to bridge real-world businesses, such as decentralized on&off-ramps, Infrastructure as a Service, freelancing or sharing economy platforms among others.
Creating infrastructure without usage is pointless, that’s why together with a few partners we have been prototyping for a while the first communities that will launch on Kreivo besides Virto:

- Cubo: Makes investment in a diversified portfolio of real state easy with the CUBO token. The team was selected in the [PolkadotRelayers program](https://www.polkadotglobalseries.com/incubator/) 3 and is currently launching a [pilot program](https://tally.so/r/3NX1DB) 2 to deploy affordable tiny houses in a touristic hotspot near Medellin Colombia.
- SwapCash: A localbitcoins-like platform to trade local currency in a decentralized way with the simplicity of Uniswap.
- Bloque: “Insta-stores” allow users to import their catalog of products from social media platforms to start selling in few clicks while receiving payments in local currency(powered by SwapCash).

### Building Kreivo MVP

Kreivo’s inner workings and initial functionality revolve around 2+1 key components:

- Communities pallet: We created a general purpose “DAO factory”/multi-collective pallet that allows local and commercial communities to establish their digital infrastructure and make decisions autonomously. Kreivo serves as their shared back-end, empowering them to conduct their operations, including payment processing.
- Payments pallet and fee collection system: Kreivo’s payments module ensures secure reversible transactions and dispute resolution. It also includes a configurable fee collection system that allows communities charge their fees or for the protocol to implement the local contributions or other impactful initiatives.
**Governance**: Kreivo adopts a “ranked collective” approach for protocol level decision-making, every community joins said collective and is able to actively participate in the governance of the system. This unique approach ensures inclusivity, prevents power concentration, and aligns with Kreivo’s mission of promoting a user-driven, accessible platform.

### The Virto Team as an Agent of the Kusama Ecosystem
The Virto team is committed to being a valuable asset to the Kusama community, offering our technical expertise as a Rust+Substrate devshop to support various projects and initiatives within the ecosystem. The team’s experience in building products that seamlessly integrate blockchain technology with user-friendly interfaces sets us apart. We understand the importance of simplifying Web3 technology for communities and end-users, making it accessible and free from unnecessary complexity.

Next Steps: The Virto team is excited to continue building in Kusama where we have established our home and hope the community sees value in supporting our [next set of endeavors](https://github.com/orgs/virto-network/projects/11/views/1) 4. We wouldn’t be able to realize the vision of Virto and Kreivo elsewhere.
Despite market challenges this year and having downsized the team we managed to extend its runway to cover an entire year of development while delivering things than initially planned, we’ve remained transparent about budgeting keeping funds on-chain(probably the only team) and staking unused funds to secure the network.
It’s our goal to make Kreivo self-sustainable as soon as possible and we are working on it but you’ll probably hear from us in Kusama’s OpenGov at least once more.

:information_source: **Join us!**: If you have a community that is looking to conduct an economic activity, we invite you all to reach out so we can set up your community in Kreivo, we are offering the first **15 communities free infrastructure** for a year which includes our own flavor of a dedicated Matrix server(what Element connects to) for encrypted chats and “light contracts” integrated with the parachain and our own Matrix client created from scratch to integrate simple payments and governance.

#### Deeper dive into Kreivo’s internals

### Governance
Kreivo introduces a novel approach to blockchain governance, diverging from the conventional token-based model. Instead, it makes use of the available “ranked collective” system, where diverse communities within the chain join forces to collectively make key decisions. This inclusive framework ensures that governance is not solely influenced by token holdings but rather by the vested interests of the active communities.

Virto, as a commercial community, is a part of said collective, contributing alongside the many other community voices. Notably, the Kusama network’s interests would be represented through the “ksm.community”, a dedicated seat in the collective. This governance structure is designed to mitigate the dominance of ‘whales’ or large token holders, fostering a more equitable and community-focused decision-making process. End users keep maintaining influence over the network’s direction through their respective community’s governance mechanisms, ensuring a broad representation of interests and a robust defense against unilateral takeovers.

### Community governance
Each community within Kreivo enjoys the autonomy to craft its unique governance structure, thanks to the flexibility of the pallet-communities. This component seamlessly integrates with other elements of the FRAME framework, such as offering a diverse set of runtime origins, managing a community from remote XCM origins and leveraging the power of the pallet-referenda.

> The team’s contributions have allowed shaping the referenda pallet to support dynamic tracks enhancing its utility for the 
> diverse and always changing community governance models.

The voice of a community is configured to be any one origin(except root and none), pallet-communities offers a varied choice of origins that allow representing all kinds of subsets of the community with different decision mechanisms. As communities grow they can also have different origins with their respective governance tracks to control different aspects of their organization such as managing memberships, controlling tokens and their supply, fees charged if there is an offering of products or services and many more.

The team will be working closely with the ecosystem via the Polkadot fellowship to allow Kreivo communities be first-class citizens across the ecosystem, this integrations will allow a community or its subsets to have representation across the entire network as a [Plurality](https://paritytech.github.io/xcm-docs/fundamentals/multilocation/junction.html#plurality) 1 that can do all kinds of actions like participating directly in the relay-chain’s governance, do staking, control remote assets, use other parachain services and so on.

### Payments
Kreivo introduces a practical and secure payment system in the blockchain sphere, akin to familiar credit card mechanisms. Focused on secure transactions and dispute resolution, it provides reassurance and options for reimbursement, similar to established financial services. The system is adaptable, featuring a configurable fee structure that empowers communities to set their own transaction fees.

#### Transaction Flow:

1. Payment Initiation: Senders initiate payments specifying the reason and amount.
2. On-Hold State: Payments arrive in an “on-hold” state, restricting fund transfers.
3. Release of Funds: If the sender receives the expected product or service, they can issue a release, freeing the on-hold funds and getting back the “release incentive”(similar to a cashback).
4. Cancellation Options: Beneficiaries can cancel the payment to return funds, and senders can request a refund.
5. Dispute Resolution: In case of disagreement, the payment enters a dispute state, and a dispute resolver is assigned to dictate fund allocation.
   
**Fee Customization System**: Kreivo’s pallet-payments also features a customizable fees & discounts system, allowing a separate subsystem to set specific fees and discounts for each payment. This includes fixed system fees like the “local incentive” or fees configured by a pallet-community-fees for community-specific transactions.

In summary, Kreivo’s payment system, with its escrow-like functionalities, payment requests, and upcoming subscription features, offers a robust solution for diverse business transaction needs in the blockchain space. We look forward to the diverse implementations by communities and welcome detailed feedback to refine and tailor this payments infrastructure to the specific requirements of every commercial community.

### Usability beyond wallets
The evolution of Web3 has brought with it a steep learning curve, especially in terms of wallet usage and the associated complexities. Users are often required to navigate through the intricacies of setting up and managing digital wallets, as well as understanding browser extensions, which can be daunting for those new to the space. Additionally, the necessity to sign and pay fees for every interaction with the blockchain adds another layer of complexity and potential deterrent for widespread adoption. These usability challenges contradict what should be a fundamental aspect of any technology: user-friendliness. We see a path where experience with a blockchain can be enhanced to the point it is no longer evident there is a blockchain behind a service, a path where crypto wallets might just become a distant memory.

#### Pallet Pass

The “pallet pass” is in Kreivo streamlines user interaction with blockchain technology. It allows users to control an on-chain account through multiple devices, leveraging a system akin to pallet-proxy.
A key feature is enabling users to use their existing [passkeys](https://passkeys.dev/) or built-in authenticators to create an account and approve transactions using native device features like fingerprints or facial recognition or a single tap of a YubiKey-like device. Pass integrates with the memberships system of the Kreivo communities, so members of a community who own an NFT-like membership, users can conduct blockchain transactions **without incurring fees** or be remembered to not have to constantly approve transactions. This approach simplifies blockchain transactions, removing the need for traditional wallet management and making the process more user-friendly.


3. Virto proposals

# Referenda #342

The Virto team, deeply committed to combating social inequality, has focused on developing tools that streamline the creation of decentralized applications for businesses. Initially funded to achieve this aim, our efforts center on building decentralized infrastructure that empowers communities to operate real world businesses efficiently and cost-effectively, contributing to local economic development.

With the support of previous funding we developed different client side tools that are now taking the shape of the VirtoSDK, a comprehensive toolkit that simplifies interactions with Substrate-based chains to the point end users don’t need to have blockchain or Web3 specific knowledge.

Isolated tools with no usage are meaningless, that's why upon this foundation we are also building Kreivo, a common-good parachain that serves as the primary platform for experimenting with and testing these tools. Kreivo offers specialized functionality for managing decentralized communities and facilitating secure payments, providing a robust platform ideal for community-driven applications and real world financial transactions. We will keep expanding the capabilities of Kreivo by implementing functionality like the Local Incentives Protocol, our initial motivation to start building in the ecosystem, that will allow the emergence of local communities which will have resources at their disposal to boost their local economy. This strategic development of Kreivo, along with the VirtoSDK, demonstrates our commitment to making blockchain technology more practical and accessible for a wide range of applications.

The Virto team, aspiring to be a leading success in the Kusama ecosystem, is here to stay, aiming to pioneer innovation and creatively use the ecosystem's technologies. We seek the community's support one more: for our long-term commitment to Kusama, our leadership in innovation, our team's expertise in Rust and Substrate, our dedication to working closely with the community to address their needs, and our focus on developing practical, user-friendly solutions. By supporting us, the community gains a partner deeply invested in advancing the ecosystem through collaborative and technically proficient initiatives.

FAQ
What experience does the Virto team have?
We have a skilled team of Rust and Substrate developers with several decades of combined software development experience, several PBA alumni, ex-Parity and Fellowship candidates. Daniel the team lead is member of the Polkadot fellowship, has 15+years of software development and computer security experience, 6+ years using Rust and 4 years building with Substrate. Other team members are even more awesome ;)

What are the expected outcomes of the funding?
The team will continuously deliver new features and updates(based on our roadmap) to the VirtoSDK, VirtoApp and Kreivo which includes a multitude of pallets and changes that we will try to upstream to repositories like the PolkadotSDK via the Polkadot Fellowship.

How will the Virto Team engage with and contribute to the Kusama community?
We will engage with the Kusama community through regular updates and establishing open feedback channels. We welcome expert curators to review our work on regular basis(e.g. quarterly) and if deemed necessary request to stop the continuous funding.

How will the team ensure transparency and accountability?
In 2023 we have already shown an exemplary management of our treasury funds which outlived their initial 6 months scope, we kept the team's KSM on-chain to secure the network, team members agreed to be payed in KSM even using the vesting pallet(which came with losses) and every expense was annotated for maximum transparency.

This year we would take things a step further, funds would be payed to the Kreivo parachain sovereign account and with through the chain's local governance and advanced payment capabilities funds would be managed openly.

Are there any long-term plans beyond this funding?
Our main goal of 2024 beyond delivering the outlined projects, is to attract as many communities to the ecosystem as possible, specially those with a real world commercial use cases. The Virto team will be one more commercial community in Kreivo that will operate as an Infrastructure as a Service platform or better yet as a Decentralized Business as a Service provider that offers organizations everything they need to go form idea to revenue generating business in a fraction of the cost and time than a traditional start-up usually takes.

In other words we want to become sustainable and hopefully not have to come back for treasury funding in the future.

How to cancel the schedule?
Anyone in the community can create a referendum that calls thescheduler.cancelNamed like in this example

Will such a big amount empty the treasury?
First of all, it's only a "big amount" if the community allows it, it could well be a short lived fund if the team doesn't deliver. We chose the recurring payment option not only to give the community more control over how long the team should be funded but more importantly to be in sync with the treasury spend cycle to get funds from what would otherwise be burned KSM. TL;DR: The Virto continuous funding doesn't impact the size of the treasury.

## Full proposal

Proposal: VirtoSDK
Proponent: EDVirtoiVjAv8HFZwSGbiBcY6P4kDFba9VKsBS911Aw8evX
Beneficiary: paraid:2281
Date: February 2024
Requested KSM:  333 KSM every spend period (6 days)
Track: Treasurer
Short description: The Virto team continues developing common good infrastructure focused on real-world adoption, user & developer experience!

Context of the proposal
The Virto team, deeply committed to combating social inequality, has focused on developing tools that streamline the creation of decentralized applications for businesses. Initially funded to achieve this aim, our efforts center on building decentralized infrastructure that empowers communities to operate real world businesses efficiently and cost-effectively, contributing to local economic development.
With the support of previous funding we developed different client side tools that are now taking the shape of the VirtoSDK, a comprehensive toolkit that simplifies interactions with Substrate-based chains to the point end users don’t need to have blockchain or Web3 specific knowledge.
Isolated tools with no usage are meaningless, that's why upon this foundation we are also building Kreivo, a common-good parachain that serves as the primary platform for experimenting with and testing these tools. Kreivo offers specialized functionality for managing decentralized communities and facilitating secure payments, providing a robust platform ideal for community-driven applications and real world financial transactions. We will keep expanding the capabilities of Kreivo by implementing functionality like the [*Local Incentives Protocol*](<https://virto.network/docs/whitepaper>), our initial motivation to start building in the ecosystem, that will allow the emergence of local communities which will have resources at their disposal to boost their local economy. This strategic development of Kreivo, along with the VirtoSDK, demonstrates our commitment to making blockchain technology more practical and accessible for a wide range of applications.

The Virto team, aspiring to be a leading success in the Kusama ecosystem, is here to stay, aiming to pioneer innovation and creatively use the ecosystem's technologies. We seek the community's support one more: for our long-term commitment to Kusama, our leadership in innovation, our team's expertise in Rust and Substrate, our dedication to working closely with the community to address their needs, and our focus on developing practical, user-friendly solutions. By supporting us, the community gains a partner deeply invested in advancing the ecosystem through collaborative and technically proficient initiatives.
Previous work
Our relationship with the Kusama ecosystem is anchored in active development and collaboration. Utilizing ecosystem funds, we've been building within Kusama for years, aspiring to become a Rust devshop dedicated to the community needs while also contributing to the Polkadot codebase.

### Virto MVP

In 2023, our team overcame numerous challenges and achieved significant successes, setting a new standard in the Kusama ecosystem. We launched a range of advanced components for the ecosystem, enhancing the experience for both Web2 developers and blockchain newcomers. This achievement marks a leap forward in innovation and progress.

Beyond technological advancements, we made a tangible impact, especially in Latin America. Our projects were more than just technical feats; they were steps towards a more inclusive and technologically empowered future. As we move into 2024, these milestones are the foundation for an exciting journey ahead, and we invite you to join us as we continue to innovate and lead in the digital era.

Team Formation: This year, we successfully assembled a highly skilled team from scratch, unified by the vision of developing user-friendly tools on the Kusama network to make Web3 accessible for everyday people. Our team's expertise was further bolstered through specialized training at the Blockchain Academy, ensuring we stay at the forefront of technical innovation.

Kreivo Blockchain Launch: We proudly launched the Kreivo parachain on Kusama, serving as a cradle of innovation for the Virto ecosystem. This launch laid the groundwork for exciting future features like community-driven initiatives and payment solutions, while maintaining our commitment to openness and common good.

Virto Super App: Our vision of how people will interact with future Web3 applications came to life with the Virto Super App. Designed with a natural, conversation-like interface, this app is a breakthrough in user experience, enabling even those with no technical background to effortlessly engage with Web3 applications.

Insightful User Research: We conducted successful tests with everyday users, builders, and merchants, proving that our technology is not only user-friendly but also adds real value. This research has been integral to the development and refinement of our toolset, ensuring our solutions meet actual user needs.

Evolution of Developer Tools: Over the past year, we've iterated and evolved our initial set of Web3 tools, enhancing their compatibility with various platforms. This evolution has significantly increased accessibility to blockchain technology, opening new avenues for innovation and participation.

Cubo.land: A first of its kind community launching on Kreivo, Cubo.land, has made strides in simplifying real estate investments. Our achievements in this domain led to our participation in the Polkadot relayers program, providing a unique platform to showcase our work. This has not only elevated our presence but also attracted more people to our ecosystem and the broader Kusama network.

### Projects Overview

We have created a technology stack that integrates client side tools, specialized Substrate pallets integrated in a custom parachain and infrastructure that when put together make it easy for everyday people to use and experiment with Web3 technology. Detailed information in our live roadmap on Github.
Our Focus for the next year: Projects
- Kreivo: the first implementation of Local Incentives Protocol (LIP), it brings the ecosystem closer to a non-technical audience. LIP focuses on making decentralized local businesses socially responsible and enables the collection of resources for the good of the community.
	We aim to bring an increased functionality over the year and contribute to the PolkadotSDK when necessary like:
Pallet-Payments: on-chain secure reversible payments analogs to the cards payment infrastructure that enable real world use cases to onboard easily with web3 liquidity infrastructure.
Pallet-communities: DAOs with advanced governance that integrate perfectly with the rest of the Kusama and Polkadot ecosystem.
Pallet-pass: Key component to enable walletless and feeless interactions with Substrate based blockchains. Users can control their on-chain account with multiple  WebAuthN capable devices(e.g. Fingerprint, FaceID, YubiKey).
Pallet-Geo: Efficient store of geographical areas of the planet that have their own on-chain presence and attributes.
Pallet-Matrix: Control of Matrix rooms from on-chain origins like a community governance track.

- Virto-sdk: Collection of tools to frictionlessly interact with with Substrate based chains, its Rust no-std compatible design help us bring the same tool to every platform and bring support to platforms like Kottlin, Swift, Flutter, Js, Python, React Native, Embedded (IoT): 
Core: progressive decentralization engine, that acts as runtime and glue  for the different components.
Virto ID: High level plug & play library to integrate third-party apps with Substrate chains with a Web2 feeling. It builds on top of previous tools.
Sube.
Libwallet.
SCALES.
- Ecosystem Contributions:  we are an ecosystem first team, where our strategy is to contribute to the development of Kusama and Polkadot following a regular channel of integrating functionality first in the PolkadotSDK through the Polkadot Fellowship.
- Virto App:  A hub for apps built with the VirtoSDK, it implements a friendly UI chat-like platform that can be extended with applets for the specific needs of each community. LLM/Assistant based interaction is a core piece of its design.

Team

Daniel (Team Lead)
Github: https://github.com/olanod
Fellowship member. Software architect.
Pablo Dorado (Senior SWE)
Github: https://github.com/pandres95
PBA Alumni (W3), Fellowship candidate, Rust and Runtime development.
Hector Bulgarini(Senior SWE)
Github: https://github.com/hbulgarini
Ex Parity engineer, PBA, Fellowship candidate.
Brayan Vargas (Mid-level SWE)
Github: https://github.com/b-avb
Rust Full-stack developer.
Ailén Grimaldi (Junior SWE)
Github: https://github.com/ail3ngrimaldi
Junior Rust developer.
Johan Duque(Senior embedded eng.)
Github: https://github.com/johandroid
PBA Alumni (W4). Fellowship candidate. Linux and Embedded engineer.
David Barinas (Senior SWE)
Github: https://github.com/S0c5
Senior Rust Full-Stack. Cybersec. Former YCombinator founder. 
Nicolas Bari(DevOps)
Github: https://github.com/nbari
Automation/Cloud/Shared/Distributed computing. Rust as the primary language.
Isabella Paz
LinkedIn: linkedin.com/in/isabellapazpaz
Communications, Marketing.
Juan Pablo Gutierrez
PBA (W4).
<TBA - PBA Substrate Dev>
<TBA - Business Development>

### Monthly Budget and spend process:  
The team will set up an automated process to convert received KSM to the Kreivo parachain sovereign account via XCM or with the help of an “OTC community” operating in Kreivo using the payments pallet. 

Like in 2023 when the team’s budget got reduced by half, the team will adapt to negative market conditions by having some members work part-time(delaying some deliverables). If market conditions change positively we will be able to pay salaries in full and any surplus would be saved as a buffer in the chain’s treasury account where it can be staked to secure the network or returned to the Kusama treasury.

With an average workload of 32 hours a week, we allow(and encourage) team members to earn extra income working in their own projects that can become Kreivo communities or in on-chain organizations like the Polkadot Fellowship.

Summary
Average cost FTE: 6 800 USD
Average total monthly cost 10-12 FTE: 68 000 - 81 600  USD/month
Estimated received KSM: 333 ,33KSM*38USD = 12666.54 * 5 = 63 332 USD/month


# Referenda 358

Following the feedback from ref#342 and the almost successful ref#349 we've decided to go for a simpler proposal that doesn't use the confusing recurring payment structure via scheduler on the Treasurer track.

This proposal is a 3 month budget(Q2) with a reduced and focused scope, to put the many tools and libraries we've developed the past year to work in the Kreivo parachain to make the simplest yet most powerful "DAO factory" in the ecosystem where users will be able to conduct real-world businesses thanks to its advanced payment system. Concretely we look to on-board 5 to 10 new DAOs(communities) by the end of Q2 to help us gather important feedback that help us build a product the Kusama community but more importantly regular users will be happy to use.

The team will also use the funds to contribute to the core Polkadot protocol via the technical Fellowship in topics we've become quite proficient like governance improvements.

Budget: 1 FTE ~= 6,500 USD | 1 Month * 9 FTE = 58,500 USD | 3 month(9 FTE) = 175,500 USD.
Asked: 3,333 KSM * 50.54 USD/KSM(EMA30) = 168,449.82 USD.

State: Executed.
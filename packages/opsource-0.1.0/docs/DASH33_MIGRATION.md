# Dash33 Repository Migration Plan

## Current State Analysis

### 1. Repository Locations
1. Main Repository:
   - Path: `/home/portiam/OPSource/dash33`
   - Status: Active, contains full implementation
   - Branch: main
   - Latest Commit: Contains Web5 integration components

2. Anya Core Submodule:
   - Path: `/home/portiam/OPSource/anya-core/dash33`
   - Status: Duplicate of main repository
   - Branch: main
   - Latest Commit: Contains same Web5 integration

3. Enterprise Integration:
   - Path: `/home/portiam/OPSource/anya-core/enterprise/dash33`
   - Status: Empty directory
   - Purpose: Intended for enterprise-specific integrations

4. Enterprise Package:
   - Path: `/home/portiam/OPSource/anya-core/enterprise/packages/dash33`
   - Status: Active Dart package
   - Contents: `.dart_tool`, `pubspec.lock`
   - Purpose: Platform-specific Dart implementation

5. Mobile Integration:
   - Path: `/home/portiam/OPSource/anya-core/mobile/lib/src/core/dash33`
   - Status: Active
   - Contents: `dash33_service.dart`
   - Purpose: Mobile-specific integration service

### 2. Dependencies and Integration Points

1. Core Dependencies:
   - Python packages (requirements.txt)
   - Rust crates (Cargo.toml)
   - Dart packages (pubspec.yaml)

2. Integration Points:
   - Web5 Data Management
   - AI Model Monitoring
   - Mobile Services
   - Enterprise Features

### 3. Current Issues

1. Code Duplication:
   - Identical code between main repo and anya-core submodule
   - Risk of divergent development
   - Maintenance overhead

2. Structural Issues:
   - Empty directories
   - Inconsistent submodule configuration
   - Mixed relative/absolute paths

3. Version Control:
   - Submodule reference issues
   - Complex update patterns
   - Potential circular dependencies

## Migration Plan

### Phase 1: Repository Consolidation

1. Main Repository Setup:
   ```bash
   # 1. Clean up main repository
   cd /home/portiam/OPSource/dash33
   git checkout main
   git pull origin main

   # 2. Create platform-specific directories
   mkdir -p platforms/{dart,mobile}
   mkdir -p integrations/{enterprise,web5}
   ```

2. Code Migration:
   - Move Dart package to `platforms/dart`
   - Move Mobile service to `platforms/mobile`
   - Move Enterprise features to `integrations/enterprise`
   - Move Web5 components to `integrations/web5`

### Phase 2: Clean Up Duplicates

1. Remove Duplicate Repositories:
   ```bash
   # 1. Remove anya-core duplicate
   cd /home/portiam/OPSource/anya-core
   git rm -rf dash33
   git commit -m "refactor: Remove duplicate dash33 directory"

   # 2. Remove empty enterprise directory
   git rm -rf enterprise/dash33
   git commit -m "chore: Remove empty dash33 directory"
   ```

2. Update References:
   - Update import paths in all dependent code
   - Update documentation references
   - Update CI/CD configurations

### Phase 3: Submodule Structure

1. Add Main Repository as Submodule:
   ```bash
   # In anya-core
   git submodule add https://github.com/botshelomokoka/dash33.git
   git config -f .gitmodules submodule.dash33.branch main
   ```

2. Configure Platform-specific References:
   ```bash
   # In enterprise package
   git config -f .gitmodules submodule.dash33.path ../../dash33
   
   # In mobile package
   git config -f .gitmodules submodule.dash33.path ../../../dash33
   ```

### Phase 4: Integration Testing

1. Test Plan:
   - Verify all imports work correctly
   - Test platform-specific features
   - Validate enterprise integration
   - Check Web5 functionality

2. Validation Steps:
   - Run full test suite
   - Verify build process
   - Test deployment procedures
   - Validate documentation

### Phase 5: Deployment

1. Deployment Steps:
   - Update production configurations
   - Deploy updated dependencies
   - Monitor for issues
   - Update documentation

2. Rollback Plan:
   - Keep backup of old structure
   - Document reversion steps
   - Maintain old references temporarily

## Timeline

1. Phase 1 (Day 1-2):
   - Repository consolidation
   - Initial structure setup

2. Phase 2 (Day 3):
   - Remove duplicates
   - Update references

3. Phase 3 (Day 4):
   - Set up new submodule structure
   - Configure dependencies

4. Phase 4 (Day 5-6):
   - Integration testing
   - Issue resolution

5. Phase 5 (Day 7):
   - Production deployment
   - Monitoring and validation

## Success Criteria

1. Technical:
   - Single source of truth for dash33 code
   - Clean submodule structure
   - All tests passing
   - No duplicate code

2. Operational:
   - Simplified maintenance
   - Clear update path
   - Documented procedures
   - Reduced complexity

3. Development:
   - Improved developer experience
   - Clear dependency structure
   - Efficient workflow
   - Better code organization

## Risk Mitigation

1. Code Preservation:
   - Create backups before migration
   - Document all changes
   - Maintain version history

2. Integration Issues:
   - Comprehensive testing plan
   - Staged rollout
   - Monitoring strategy

3. Team Coordination:
   - Clear communication plan
   - Team training sessions
   - Documentation updates

## Post-Migration Tasks

1. Documentation:
   - Update README files
   - Update API documentation
   - Create migration guide

2. Cleanup:
   - Remove temporary files
   - Archive old configurations
   - Update CI/CD pipelines

3. Monitoring:
   - Track performance metrics
   - Monitor error rates
   - Collect feedback

## Team Responsibilities

1. Repository Admin:
   - Lead migration process
   - Coordinate with teams
   - Validate changes

2. Development Teams:
   - Update dependent code
   - Test integrations
   - Report issues

3. DevOps:
   - Update build processes
   - Configure CI/CD
   - Monitor deployment

4. QA Team:
   - Validate changes
   - Run test suites
   - Report issues

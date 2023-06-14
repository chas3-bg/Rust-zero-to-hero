str = "iotfleetwise:ListSignalCatalogs, iotanalytics:ListTagsForResource, lookoutmetrics:ListAnomalyDetectors, iotanalytics:ListChannels, lookoutequipment:ListDatasets, servicediscovery:ListNamespaces, opsworks:DescribeUserProfiles, lookoutvision:ListProjects, polly:DescribeVoices, swf:ListDomains, comprehendmedical:ListEntitiesDetectionV2Jobs"
pard = str.split(",")
for i in pard:
    print('"' + i.strip() + '",')
